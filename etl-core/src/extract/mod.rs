use arrow::datatypes::Schema;
use arrow::record_batch::RecordBatch;
use async_trait::async_trait;
use serde::de::DeserializeOwned;
use std::collections::HashMap;

pub mod error;
pub mod rest_extractor;

pub type ExtractorResult<T> = Result<T, error::ExtractorError>;
pub type Metadata = HashMap<String, String>;
pub struct Checkpoint(pub String);

#[async_trait]
pub trait Extractor {
    // Lifecyce functions
    // Build a standard init() fn
    async fn ping(&self) -> ExtractorResult<()>;
    async fn close() -> ExtractorResult<()>;
    // Data Retrieval
    async fn extract<T: DeserializeOwned>(&self) -> ExtractorResult<T>;
    async fn to_record_batch(&self) -> ExtractorResult<RecordBatch>;
    // This needs to implement RecordBatchStream
    //async fn into_stream() -> ExtractorResult<BoxStream<Self::Item>>;
    async fn extract_batch<T>() -> ExtractorResult<Vec<T>>;
    // Schema/Metadata
    fn schema() -> Option<Schema> {
        None
    }
    fn source_name(&self) -> ExtractorResult<&str>;
    async fn metadata(&self) -> ExtractorResult<Metadata>;
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
