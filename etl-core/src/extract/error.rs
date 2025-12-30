use datafusion::arrow::error::ArrowError;
use datafusion::error::DataFusionError;
use object_store::Error as ObjStoreError;
use reqwest::Error as ReqwestError;
use serde_json::Error as SerdeError;
use std::io::Error as IoError;
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

    #[error("DataFusion error: {0}")]
    DataFusionError(#[from] DataFusionError),

    #[error("Standard Error: {0}")]
    StandardError(#[from] IoError),

    #[error("Object Store Error: {0}")]
    ObjectStoreError(#[from] ObjStoreError),

    #[error("Arrow error: {0}")]
    ArrowError(#[from] ArrowError),
}
