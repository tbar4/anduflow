use serde::de::DeserializeOwned;

use crate::extract::{Extractor, ExtractorResult};

use super::error::ExtractorError;
use reqwest::{Client, Request, RequestBuilder};

#[derive(Debug)]
pub struct RestExtractor {
    client: Client,
    request: RequestBuilder,
}

impl RestExtractor {
    pub fn new(base_url: &str, endpoint: &str) -> Self {
        let trimmed_base = base_url.trim_end_matches('/');
        let trimmed_endpoint = endpoint.trim_start_matches('/');
        let rest_api = format!("{}/{}", trimmed_base, trimmed_endpoint);

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

    pub fn build_request(self) -> ExtractorResult<Request> {
        Ok(self.request.build()?)
    }
}

#[async_trait::async_trait]
impl Extractor for RestExtractor {
    async fn extract<T: DeserializeOwned>(&self) -> ExtractorResult<T> {
        let request = self.request
            .try_clone()
            .ok_or(ExtractorError::RequestCloneFailed)?
            .build()?;
        let response = self.client.execute(request).await?;
        let parsed = response.json::<T>().await?;
        Ok(parsed)
    }

    fn get_config(&self) -> std::collections::HashMap<String, String> {
        std::collections::HashMap::new()
    }

    fn get_metadata(&self) -> std::collections::HashMap<String, String> {
        std::collections::HashMap::new()
    }
}