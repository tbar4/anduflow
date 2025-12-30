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
etl-core = "0.1"
```

Or if you want to use the latest version from the repository:

```toml
[dependencies]
etl-core = { git = "https://github.com/your-username/anduflow", branch = "main" }
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

For online documentation, visit [docs.rs/etl-core](https://docs.rs/etl-core).