//! Traits and types for data extraction in ETL pipelines.
//!
//! This module provides the core traits and types for extracting data from various sources.
//! The main trait is [`Extractor`], which defines the interface for all extractors.
//!
//! # Examples
//!
//! ```
//! use anduflow_core::extract::{Extractor, ExtractorResult, rest_extractor::RestExtractor};
//!
//! #[tokio::main]
//! async fn example() -> ExtractorResult<()> {
//!     let extractor = RestExtractor::new("https://api.example.com", "data");
//!     let data: serde_json::Value = extractor.extract().await?;
//!     Ok(())
//! }
//! ```

use async_trait::async_trait;
use serde::de::DeserializeOwned;
use bytes::Bytes;
use anduflow_utils::error;
use anduflow_utils::error::ExtractorResult;
use anduflow_utils::logger::store::LogStore;

pub mod rest_extractor;



/// A checkpoint for incremental extraction.
#[derive(Debug, Clone)]
pub struct Checkpoint(pub String);

/// Format for data extraction.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ExtractFormat {
    /// JSON format
    Json,
    /// Plain text format
    Text,
    /// Raw bytes format
    Bytes,
}

/// The main trait for all extractors.
///
/// This trait defines the interface for extracting data from various sources.
/// Implementors should provide implementations for the required methods and
/// can override the default implementations for optional methods.
///
/// # Required Methods
///
/// - [`ping`](Extractor::ping): Check if the source is available
/// - [`close`](Extractor::close): Close the extractor and release resources
/// - [`extract_json`](Extractor::extract_json): Extract data as JSON
/// - [`extract_text`](Extractor::extract_text): Extract data as text
/// - [`extract_bytes`](Extractor::extract_bytes): Extract data as bytes
/// - [`extract_raw`](Extractor::extract_raw): Extract data as raw bytes
/// - [`source_name`](Extractor::source_name): Get the name of the source
/// - [`metadata`](Extractor::metadata): Get metadata about the source
///
/// # Default Methods
///
/// - [`extract`](Extractor::extract): Extract data as JSON (convenience wrapper)
/// - [`schema`](Extractor::schema): Get the schema of the source (default: None)
/// - [`supports_incremental`](Extractor::supports_incremental): Check if incremental extraction is supported (default: false)
/// - [`checkpoint`](Extractor::checkpoint): Get the current checkpoint (default: None)
/// - [`set_checkpoint`](Extractor::set_checkpoint): Set the checkpoint (default: error if not supported)
#[async_trait]
pub trait Extractor {
    // Lifecycle functions
    // Build a standard init() fn
    /// Check if the source is available.
    ///
    /// This method should perform a lightweight check to verify that the source
    /// is accessible and responding correctly.
    ///
    /// # Returns
    ///
    /// - `Ok(())` if the source is available
    /// - `Err(ExtractorError)` if the source is not available or an error occurred
    async fn ping(&self) -> ExtractorResult<()>;
    
    /// Close the extractor and release resources.
    ///
    /// This method should clean up any resources used by the extractor,
    /// such as network connections or file handles.
    ///
    /// # Returns
    ///
    /// - `Ok(())` if the extractor was closed successfully
    /// - `Err(ExtractorError)` if an error occurred while closing
    async fn close() -> ExtractorResult<()>;
    
    // Data Retrieval
    /// Extract data from the source as JSON.
    ///
    /// This is a convenience method that calls [`extract_json`](Extractor::extract_json).
    ///
    /// # Type Parameters
    ///
    /// - `T`: The type to deserialize the JSON data into. Must implement `DeserializeOwned`.
    ///
    /// # Returns
    ///
    /// - `Ok(T)` with the deserialized data
    /// - `Err(ExtractorError)` if an error occurred during extraction or deserialization
    async fn extract<T: DeserializeOwned>(&self, logger: &mut LogStore) -> ExtractorResult<T> {
        self.extract_json(logger).await
    }
    
    /// Extract data from the source as JSON.
    ///
    /// This method should fetch data from the source and deserialize it as JSON.
    ///
    /// # Type Parameters
    ///
    /// - `T`: The type to deserialize the JSON data into. Must implement `DeserializeOwned`.
    ///
    /// # Returns
    ///
    /// - `Ok(T)` with the deserialized data
    /// - `Err(ExtractorError)` if an error occurred during extraction or deserialization
    async fn extract_json<T: DeserializeOwned>(&self, logger: &mut LogStore) -> ExtractorResult<T>;
    
    /// Extract data from the source as text.
    ///
    /// This method should fetch data from the source and return it as a string.
    ///
    /// # Returns
    ///
    /// - `Ok(String)` with the text data
    /// - `Err(ExtractorError)` if an error occurred during extraction
    async fn extract_text(&self) -> ExtractorResult<String>;
    
    /// Extract data from the source as bytes.
    ///
    /// This method should fetch data from the source and return it as a vector of bytes.
    ///
    /// # Returns
    ///
    /// - `Ok(Vec<u8>)` with the byte data
    /// - `Err(ExtractorError)` if an error occurred during extraction
    async fn extract_bytes(&self) -> ExtractorResult<Vec<u8>>;
    
    /// Extract data from the source as raw bytes.
    ///
    /// This method should fetch data from the source and return it as raw bytes.
    ///
    /// # Returns
    ///
    /// - `Ok(Bytes)` with the raw byte data
    /// - `Err(ExtractorError)` if an error occurred during extraction
    async fn extract_raw(&self) -> ExtractorResult<Bytes>;
    
    // Schema/Metadata
    /// Get the schema of the source.
    ///
    /// This method should return a string representation of the schema of the source,
    /// if available. The default implementation returns `None`.
    ///
    /// # Returns
    ///
    /// - `Some(String)` with the schema representation
    /// - `None` if the schema is not available or not implemented
    fn schema() -> Option<String> {
        None
    }
    
    /// Get the name of the source.
    ///
    /// This method should return the name of the source that this extractor is
    /// extracting data from.
    ///
    /// # Returns
    ///
    /// - `Ok(&str)` with the source name
    /// - `Err(ExtractorError)` if an error occurred
    fn source_name(&self) -> ExtractorResult<&str>;
    
    /// Get metadata about the source.
    ///
    /// This method should return metadata about the source, such as the last
    /// modified time, size, or other relevant information.
    ///
    /// # Returns
    ///
    /// - `Ok(String)` with the metadata
    /// - `Err(ExtractorError)` if an error occurred
    async fn metadata(&self) -> ExtractorResult<String>;
    
    // Incremental/Checkpointing
    /// Check if incremental extraction is supported.
    ///
    /// This method should return `true` if the extractor supports incremental
    /// extraction, `false` otherwise. The default implementation returns `false`.
    ///
    /// # Returns
    ///
    /// - `true` if incremental extraction is supported
    /// - `false` if incremental extraction is not supported
    fn supports_incremental(&self) -> bool {
        false
    }
    
    /// Get the current checkpoint.
    ///
    /// This method should return the current checkpoint for incremental extraction,
    /// if available and if incremental extraction is supported.
    ///
    /// # Returns
    ///
    /// - `Some(Checkpoint)` with the current checkpoint
    /// - `None` if no checkpoint is available or incremental extraction is not supported
    fn checkpoint(&self) -> Option<Checkpoint> {
        None
    }
    
    /// Set the checkpoint for incremental extraction.
    ///
    /// This method should set the checkpoint for incremental extraction.
    /// If incremental extraction is not supported, it should return an error.
    ///
    /// # Parameters
    ///
    /// - `chk`: The checkpoint to set
    ///
    /// # Returns
    ///
    /// - `Ok(())` if the checkpoint was set successfully
    /// - `Err(ExtractorError)` if an error occurred or incremental extraction is not supported
    fn set_checkpoint(&mut self, _chk: Checkpoint) -> ExtractorResult<()> {
        if self.supports_incremental() {
            Ok(())
        } else {
            Err(error::ExtractorError::ExtractOpsError(
                "Source does not support incremental".into(),
            ))
        }
    }
}
