use async_trait::async_trait;
use serde::de::DeserializeOwned;
use bytes::Bytes;

pub mod error;
pub mod rest_extractor;

pub type ExtractorResult<T> = Result<T, error::ExtractorError>;
pub struct Checkpoint(pub String);

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ExtractFormat {
    Json,
    Text,
    Bytes,
}

#[async_trait]
pub trait Extractor {
    // Lifecycle functions
    // Build a standard init() fn
    async fn ping(&self) -> ExtractorResult<()>;
    async fn close() -> ExtractorResult<()>;
    
    // Data Retrieval
    async fn extract<T: DeserializeOwned>(&self) -> ExtractorResult<T> {
        self.extract_json().await
    }
    
    async fn extract_json<T: DeserializeOwned>(&self) -> ExtractorResult<T>;
    
    async fn extract_text(&self) -> ExtractorResult<String>;
    
    async fn extract_bytes(&self) -> ExtractorResult<Vec<u8>>;
    
    async fn extract_raw(&self) -> ExtractorResult<Bytes>;
    
    // Schema/Metadata
    fn schema() -> Option<String> {
        None
    }
    fn source_name(&self) -> ExtractorResult<&str>;
    async fn metadata(&self) -> ExtractorResult<String>;
    
    // Incremental/Checkpointing
    fn supports_incremental(&self) -> bool {
        false
    }
    fn checkpoint(&self) -> Option<Checkpoint> {
        None
    }
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
