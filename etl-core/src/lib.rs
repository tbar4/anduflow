//! Core ETL (Extract, Transform, Load) library for Rust.
//!
//! This library provides a framework for building ETL pipelines in Rust.
//! It includes traits and implementations for extracting data from various
//! sources, transforming it, and loading it into various destinations.
//!
//! # Features
//!
//! - Extensible extractor framework with built-in REST API support
//! - Error handling with detailed error types
//! - Asynchronous operations using Tokio
//! - Support for various data formats (JSON, text, bytes)
//!
//! # Modules
//!
//! - [`extract`]: Traits and implementations for data extraction
//!
//! # Examples
//!
//! ```
//! use etl_core::extract::{Extractor, ExtractorResult, rest_extractor::RestExtractor};
//!
//! #[tokio::main]
//! async fn example() -> ExtractorResult<()> {
//!     let extractor = RestExtractor::new("https://api.example.com", "data");
//!     let data: serde_json::Value = extractor.extract().await?;
//!     Ok(())
//! }
//! ```

pub mod extract;
