# ETL Core

Core ETL (Extract, Transform, Load) library for Rust.

## Overview

This library provides a framework for building ETL pipelines in Rust. It includes traits and implementations for extracting data from various sources, transforming it, and loading it into various destinations.

## Features

- Extensible extractor framework with built-in REST API support
- Error handling with detailed error types
- Asynchronous operations using Tokio
- Support for various data formats (JSON, text, bytes)

## Modules

- `extract`: Traits and implementations for data extraction

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
etl-core = { path = "etl-core" }
```

## Examples

```rust
use etl_core::extract::{Extractor, ExtractorResult, rest_extractor::RestExtractor};

#[tokio::main]
async fn main() -> ExtractorResult<()> {
    let extractor = RestExtractor::new("https://api.example.com", "data");
    let data: serde_json::Value = extractor.extract().await?;
    Ok(())
}
```

## Testing

To run the tests:

```bash
cargo test
```

Tests are organized in a separate `tests` directory for better maintainability:
- Unit tests for error handling in `tests/error_tests.rs`
- Integration tests for REST extractor in `tests/rest_extractor_tests.rs`

## Documentation

To generate and view the documentation:

```bash
cargo doc --open
```