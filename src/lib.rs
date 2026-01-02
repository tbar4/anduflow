//! Anduflow - ETL (Extract, Transform, Load) application for Rust.
//!
//! This crate provides a complete ETL application built on top of the
//! `anduflow_core` library. It includes command-line
//! tools and examples for performing ETL operations.
//!
//! # Features
//!
//! - Command-line interface for ETL operations
//! - Example implementations of common ETL patterns
//! - Integration with the core ETL library
//!
//! # Examples
//!
//! 
//! See the `examples` directory for example implementations of ETL pipelines.

pub mod anduflow_core {
    pub use anduflow_core::*;
}
pub mod anduflow_utils {
    pub use anduflow_utils::*;
}
