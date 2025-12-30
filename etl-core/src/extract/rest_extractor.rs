use bytes::Bytes;
use serde::de::DeserializeOwned;

use crate::extract::{Extractor, ExtractorResult};

use super::error::ExtractorError;
use reqwest::{Client, Request, RequestBuilder, Method};

#[derive(Debug)]
pub struct RestExtractor {
    client: Client,
    request: RequestBuilder,
}

impl RestExtractor {
    pub fn new(base_url: &str, endpoint: &str) -> Self {
        let trimmed_base = base_url.trim_end_matches('/');
        let trimmed_endpoint = endpoint.trim_start_matches('/');
        let rest_api = format!("{trimmed_base}/{trimmed_endpoint}");

        RestExtractor {
            client: Client::new(),
            request: Client::new().get(rest_api.as_str()),
        }
    }

    pub fn with_basic_auth(mut self, username: &str, password: &str) -> Self {
        self.request = self.request.basic_auth(username, Some(password));
        self
    }

    pub fn with_header(mut self, key: &str, value: &str) -> Self {
        self.request = self.request.header(key, value);
        self
    }

    pub fn with_query_param(mut self, query: &[(&str, &str)]) -> Self {
        self.request = self.request.query(query);
        self
    }

    pub fn with_auth_token(mut self, token: &str) -> Self {
        self.request = self.request.bearer_auth(token);
        self
    }

    /// Sets the HTTP method for the request. Note: this will recreate the request
    /// builder from the client and will not preserve previously-set query params.
    /// Call this before adding headers or query params when possible.
    pub fn with_method<S: AsRef<str>>(mut self, method: S) -> Self {
        // Accept a string method (e.g. "GET", "POST") to avoid forcing callers to depend
        // on `reqwest` just to choose a method. Unknown methods fall back to GET.
        let method_str = method.as_ref();
        let parsed = method_str.parse::<Method>().unwrap_or(Method::GET);
        let built = self.request.try_clone().unwrap().build().unwrap();
        let url = built.url().to_string();
        let headers = built.headers().clone();
        self.request = self.client.request(parsed, url.as_str());
        // Re-apply headers that were set on the previous builder
        for (name, value) in headers.iter() {
            self.request = self.request.header(name, value.clone());
        }
        self
    }

    /// Attach a raw body to the request.
    pub fn with_body<B: Into<reqwest::Body>>(mut self, body: B) -> Self {
        self.request = self.request.body(body);
        self
    }

    /// Attach a JSON body and set the appropriate Content-Type header.
    pub fn with_json_body<T: serde::Serialize>(mut self, value: &T) -> Self {
        self.request = self.request.json(value);
        self
    }

    pub fn build_request(self) -> ExtractorResult<Request> {
        Ok(self.request.build()?)
    }
    pub fn url(&self) -> String {
        self.request.try_clone().unwrap().build().unwrap().url().to_string()
    }
}

#[async_trait::async_trait]
impl Extractor for RestExtractor {
    async fn ping(&self) -> ExtractorResult<()> {
        let request = self
            .request
            .try_clone()
            .ok_or(ExtractorError::RequestCloneFailed)?
            .build()?;
        let status_code = self.client.execute(request).await?.status();
        match status_code.is_success() {
            true => {
                println!("Ping successful with status code: {status_code}");
                Ok(())
            }
            false => {
                println!("Ping failed with status code: {status_code}");
                Ok(())
            }
        }
    }
    async fn close() -> ExtractorResult<()> {
        println!("Closing RestExtractor resources.");
        Ok(())
    }
    
    async fn extract_json<T: DeserializeOwned>(&self) -> ExtractorResult<T> {
        let request = self
            .request
            .try_clone()
            .ok_or(ExtractorError::RequestCloneFailed)?
            .build()?;
        let response = self.client.execute(request).await?;
        let status = response.status();

        // Read the response body as text first so we can provide clearer errors for empty or non-JSON bodies
        let text = response.text().await?;
        if text.trim().is_empty() {
            return Err(ExtractorError::ExtractOpsError(format!(
                "Empty response body (status: {})",
                status
            )));
        }

        // Attempt to deserialize from the obtained text. If parsing fails, return a
        // clear error that includes a snippet of the response body to aid debugging.
        match serde_json::from_str::<T>(&text) {
            Ok(parsed) => Ok(parsed),
            Err(e) => {
                let snippet: String = text.chars().take(1024).collect();
                return Err(ExtractorError::ExtractOpsError(format!(
                    "Failed to parse JSON: {}. Response snippet: {}",
                    e, snippet
                )));
            }
        }
    }
    
    async fn extract_text(&self) -> ExtractorResult<String> {
        let request = self
            .request
            .try_clone()
            .ok_or(ExtractorError::RequestCloneFailed)?
            .build()?;
        let response = self.client.execute(request).await?;
        Ok(response.text().await?)
    }
    
    async fn extract_bytes(&self) -> ExtractorResult<Vec<u8>> {
        let request = self
            .request
            .try_clone()
            .ok_or(ExtractorError::RequestCloneFailed)?
            .build()?;
        let response = self.client.execute(request).await?;
        Ok(response.bytes().await?.to_vec())
    }
    
    async fn extract_raw(&self) -> ExtractorResult<Bytes> {
        let request = self
            .request
            .try_clone()
            .ok_or(ExtractorError::RequestCloneFailed)?
            .build()?;
        let response = self.client.execute(request).await?;
        Ok(response.bytes().await?)
    }
    fn source_name(&self) -> ExtractorResult<&str> {
        Ok("RestExtractor")
    }
    async fn metadata(&self) -> ExtractorResult<String> {
        unimplemented!()
    }
    fn supports_incremental(&self) -> bool {
        false
    }
    fn checkpoint(&self) -> Option<super::Checkpoint> {
        None
    }
    fn set_checkpoint(&mut self, _chk: super::Checkpoint) -> ExtractorResult<()> {
        Err(ExtractorError::ExtractOpsError(
            "Source does not support incremental".into(),
        ))
    }
}
