use arrow::error::ArrowError;
use flatten_json_object::Error as FlattenerError;
use reqwest::Error as ReqwestError;
use serde_json::Error as SerdeError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ExtractorError {
    #[error("HTTP request failed: {0}")]
    HttpRequestError(#[from] ReqwestError),

    #[error("could not clone request for execution")]
    RequestCloneFailed,

    #[error("Serialization error: {0}")]
    SerializationError(#[from] SerdeError),

    #[error("Extract Operation Error: {0}")]
    ExtractOpsError(String),

    #[error("Arrow error: {0}")]
    ArrowError(#[from] ArrowError),

    #[error("JSON Flattening error: {0}")]
    JsonFlatteningError(#[from] FlattenerError),
}
