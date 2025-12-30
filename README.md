# Rust ETL

ETL (Extract, Transform, Load) application and library for Rust.

## Overview

This project provides a complete ETL (Extract, Transform, Load) solution built in Rust. It includes both a core library for building ETL pipelines and a command-line application for performing ETL operations.

## Structure

- `etl-core`: Core ETL library with traits and implementations for data extraction
- `src`: Main application code
- `examples`: Example implementations of ETL pipelines

## Features

- Extensible extractor framework with built-in REST API support
- Error handling with detailed error types
- Asynchronous operations using Tokio
- Support for various data formats (JSON, text, bytes)
- Comprehensive documentation and tests

## Usage

### As a Library

Add this to your `Cargo.toml`:

```toml
[dependencies]
etl-core = { path = "etl-core" }
```

### As an Application

To run the application:

```bash
cargo run
```

## Examples

See the `examples` directory for example implementations of ETL pipelines.

## Testing

To run the tests:

```bash
# Run all tests
cargo test

# Run tests for the core library only
cd etl-core && cargo test
```

## Documentation

To generate and view the documentation:

```bash
# Generate documentation for the entire workspace
cargo doc --open

# Generate documentation for the core library only
cd etl-core && cargo doc --open
```
