//! Error types for the extract module.
//!
//! This module defines the error types that can occur during extraction operations.
//! All extractors should return errors of type [`ExtractorError`] or a type that
//! can be converted to it.

use datafusion::arrow::error::ArrowError;
use datafusion::error::DataFusionError;
use object_store::Error as ObjStoreError;
use reqwest::Error as ReqwestError;
use serde_json::Error as SerdeError;
use std::io::Error as IoError;
use rusqlite::Error as RusqliteError;
use thiserror::Error;

/// The result type for extractor operations.
pub type ExtractorResult<T> = Result<T, ExtractorError>;

/// The error type for extractor operations.
///
/// This enum represents all the possible errors that can occur during
/// extraction operations. It uses the `thiserror` crate to provide
/// automatic implementations of `std::error::Error` and `std::fmt::Display`.
#[derive(Error, Debug)]
pub enum ExtractorError {
    /// An HTTP request failed.
    ///
    /// This variant wraps a `reqwest::Error` and is used when an HTTP request
    /// fails for any reason, such as network issues, timeouts, or HTTP error
    /// status codes.
    #[error("HTTP request failed: {0}")]
    HttpRequestError(#[from] ReqwestError),

    /// Could not clone request for execution.
    ///
    /// This error occurs when trying to clone a request builder fails.
    /// This is typically an internal error.
    #[error("could not clone request for execution")]
    RequestCloneFailed,

    /// Serialization or deserialization error.
    ///
    /// This variant wraps a `serde_json::Error` and is used when JSON
    /// serialization or deserialization fails.
    #[error("Serialization error: {0}")]
    SerializationError(#[from] SerdeError),

    /// Extract operation error.
    ///
    /// This is a generic error for extract operations that don't fit into
    /// the other categories. The string contains a description of the error.
    #[error("Extract Operation Error: {0}")]
    ExtractOpsError(String),

    /// DataFusion error.
    ///
    /// This variant wraps a `datafusion::error::DataFusionError` and is used
    /// when DataFusion operations fail.
    #[error("DataFusion error: {0}")]
    DataFusionError(#[from] DataFusionError),

    /// Standard I/O error.
    ///
    /// This variant wraps a `std::io::Error` and is used when standard I/O
    /// operations fail.
    #[error("Standard Error: {0}")]
    StandardError(#[from] IoError),

    /// Object store error.
    ///
    /// This variant wraps an `object_store::Error` and is used when object
    /// store operations fail.
    #[error("Object Store Error: {0}")]
    ObjectStoreError(#[from] ObjStoreError),

    /// Arrow error.
    ///
    /// This variant wraps an `arrow::error::ArrowError` and is used when
    /// Arrow operations fail.
    #[error("Arrow error: {0}")]
    ArrowError(#[from] ArrowError),

    /// SQLite error.
    /// 
    /// This variant wraps a `rusqlite::Error` and is used when SQLite operations fail.
    #[error ("SQLite error: {0}")]
    SqliteError(#[from] RusqliteError),
}
