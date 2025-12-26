use thiserror::Error;
use reqwest::Error as ReqwestError;
use serde_json::Error as SerdeError;

#[derive(Error, Debug)]
pub enum ExtractorError {
    #[error("HTTP request failed: {0}")]
    HttpRequestError(#[from] ReqwestError),

    #[error("could not clone request for execution")]
    RequestCloneFailed,

    #[error("Serialization error: {0}")]
    SerializationError(#[from] SerdeError),
}