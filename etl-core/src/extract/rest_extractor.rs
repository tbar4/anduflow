use arrow::array::RecordBatch;
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

    pub fn build_request(self) -> ExtractorResult<Request> {
        Ok(self.request.build()?)
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
    async fn extract<T: DeserializeOwned>(&self) -> ExtractorResult<T> {
        let request = self
            .request
            .try_clone()
            .ok_or(ExtractorError::RequestCloneFailed)?
            .build()?;
        let response = self.client.execute(request).await?;
        let result = response.json::<T>().await?;

        Ok(result)
    }
    async fn to_record_batch(&self) -> ExtractorResult<RecordBatch> {
        unimplemented!()
    }
    async fn extract_batch<T>() -> ExtractorResult<Vec<T>> {
        unimplemented!()
    }
    fn source_name(&self) -> ExtractorResult<&str> {
        unimplemented!()
    }
    async fn metadata(&self) -> ExtractorResult<super::Metadata> {
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
