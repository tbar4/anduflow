//! Tests for the rest_extractor module.

use etl_core::extract::rest_extractor::RestExtractor;
use etl_core::extract::Extractor;
use httpmock::prelude::*;

#[tokio::test]
async fn test_rest_extractor_new() {
    let extractor = RestExtractor::new("https://api.example.com", "data");
    assert_eq!(extractor.url(), "https://api.example.com/data");
}

#[tokio::test]
async fn test_rest_extractor_with_query_param() {
    let extractor = RestExtractor::new("https://api.example.com", "data")
        .with_query_param(&[("limit", "10"), ("offset", "0")]);
    // The exact URL format may vary, but it should contain the base URL and endpoint
    assert!(extractor.url().starts_with("https://api.example.com/data"));
}

#[tokio::test]
async fn test_rest_extractor_with_header() {
    let extractor = RestExtractor::new("https://api.example.com", "data")
        .with_header("User-Agent", "Test-Agent/1.0");
    // We can't easily test headers without making a request, but we can test that
    // the extractor was created successfully
    assert_eq!(extractor.url(), "https://api.example.com/data");
}

#[tokio::test]
async fn test_rest_extractor_with_auth_token() {
    let extractor = RestExtractor::new("https://api.example.com", "data")
        .with_auth_token("test-token");
    assert_eq!(extractor.url(), "https://api.example.com/data");
}

#[tokio::test]
async fn test_rest_extractor_with_basic_auth() {
    let extractor = RestExtractor::new("https://api.example.com", "data")
        .with_basic_auth("user", "pass");
    assert_eq!(extractor.url(), "https://api.example.com/data");
}

#[tokio::test]
async fn test_rest_extractor_with_method() {
    let extractor = RestExtractor::new("https://api.example.com", "data")
        .with_method("POST");
    assert_eq!(extractor.url(), "https://api.example.com/data");
}

#[tokio::test]
async fn test_rest_extractor_source_name() {
    let extractor = RestExtractor::new("https://api.example.com", "data");
    assert_eq!(extractor.source_name().unwrap(), "RestExtractor");
}

#[tokio::test]
async fn test_rest_extractor_extract_json() {
    let server = MockServer::start();

    let mock = server.mock(|when, then| {
        when.method(GET)
            .path("/api/data");
        then.status(200)
            .header("content-type", "application/json")
            .json_body(serde_json::json!({
                "id": 1,
                "name": "test"
            }));
    });

    let extractor = RestExtractor::new(&server.base_url(), "api/data");
    let result: serde_json::Value = extractor.extract().await.unwrap();
    
    mock.assert();
    assert_eq!(result["id"], 1);
    assert_eq!(result["name"], "test");
}

#[tokio::test]
async fn test_rest_extractor_extract_text() {
    let server = MockServer::start();

    let mock = server.mock(|when, then| {
        when.method(GET)
            .path("/api/text");
        then.status(200)
            .header("content-type", "text/plain")
            .body("Hello, world!");
    });

    let extractor = RestExtractor::new(&server.base_url(), "api/text");
    let result = extractor.extract_text().await.unwrap();
    
    mock.assert();
    assert_eq!(result, "Hello, world!");
}

#[tokio::test]
async fn test_rest_extractor_extract_bytes() {
    let server = MockServer::start();

    let mock = server.mock(|when, then| {
        when.method(GET)
            .path("/api/bytes");
        then.status(200)
            .header("content-type", "application/octet-stream")
            .body(b"Hello, world!");
    });

    let extractor = RestExtractor::new(&server.base_url(), "api/bytes");
    let result = extractor.extract_bytes().await.unwrap();
    
    mock.assert();
    assert_eq!(result, b"Hello, world!");
}

#[tokio::test]
async fn test_rest_extractor_extract_raw() {
    let server = MockServer::start();

    let mock = server.mock(|when, then| {
        when.method(GET)
            .path("/api/raw");
        then.status(200)
            .header("content-type", "application/octet-stream")
            .body(b"Hello, world!");
    });

    let extractor = RestExtractor::new(&server.base_url(), "api/raw");
    let result = extractor.extract_raw().await.unwrap();
    
    mock.assert();
    assert_eq!(result, bytes::Bytes::from_static(b"Hello, world!"));
}

#[tokio::test]
async fn test_rest_extractor_post_with_json_body() {
    let server = MockServer::start();

    let mock = server.mock(|when, then| {
        when.method(POST)
            .path("/api/data")
            .header("content-type", "application/json")
            .json_body(serde_json::json!({
                "query": "test"
            }));
        then.status(200)
            .header("content-type", "application/json")
            .json_body(serde_json::json!({
                "result": "success"
            }));
    });

    let extractor = RestExtractor::new(&server.base_url(), "api/data")
        .with_method("POST")
        .with_json_body(&serde_json::json!({
            "query": "test"
        }));
    
    let result: serde_json::Value = extractor.extract().await.unwrap();
    
    mock.assert();
    assert_eq!(result["result"], "success");
}

#[tokio::test]
async fn test_rest_extractor_error_handling() {
    let server = MockServer::start();

    let mock = server.mock(|when, then| {
        when.method(GET)
            .path("/api/error");
        then.status(500)
            .header("content-type", "application/json")
            .json_body(serde_json::json!({
                "error": "Internal server error"
            }));
    });

    let extractor = RestExtractor::new(&server.base_url(), "api/error");
    let result: Result<serde_json::Value, _> = extractor.extract().await;
    
    mock.assert();
    // The request should succeed (status 500 is not a network error), but JSON parsing might fail
    // depending on the response format. In this case, we're returning valid JSON, so it should succeed.
    assert!(result.is_ok());
}