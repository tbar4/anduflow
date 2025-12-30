//! Tests for the error module.

use etl_core::extract::error::ExtractorError;
use std::io::Error as IoError;

#[test]
fn test_extract_ops_error() {
    let extractor_err = ExtractorError::ExtractOpsError("test error".to_string());
    
    match extractor_err {
        ExtractorError::ExtractOpsError(msg) => {
            assert_eq!(msg, "test error");
        },
        _ => panic!("Expected ExtractOpsError"),
    }
}

#[test]
fn test_standard_error() {
    let err = IoError::new(std::io::ErrorKind::Other, "test io error");
    let extractor_err: ExtractorError = err.into();
    
    match extractor_err {
        ExtractorError::StandardError(_) => {}, // Success
        _ => panic!("Expected StandardError"),
    }
}