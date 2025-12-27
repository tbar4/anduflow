use std::collections::HashMap;

use async_trait::async_trait;
use serde::de::DeserializeOwned;

pub mod rest_extractor;
pub mod error;

pub type ExtractorResult<T> = Result<T, error::ExtractorError>;

#[async_trait]
pub trait Extractor {
    async fn extract<T: DeserializeOwned>(&self) -> ExtractorResult<T>;
    fn get_config(&self) -> HashMap<String, String>;
    fn get_metadata(&self) -> HashMap<String, String>;
}