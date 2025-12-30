use etl_core::extract::{Extractor, ExtractorResult, rest_extractor::RestExtractor};
use dotenv::dotenv;

#[tokio::main]
async fn main() -> ExtractorResult<()> {
    dotenv().ok();
    let _perp_api_key = std::env::var("PERPLEXITY_API_KEY").expect("PERPLEXITY_API_KEY must be set in .env file");

    let spacedevs: RestExtractor =
        RestExtractor::new("https://api.spaceflightnewsapi.net/v4", "articles")
        .with_query_param(&[("updated_at_gte", "2025-12-21"), ("ordering", "-updated_at"), ("limit", "1"), ("offset", "0"), ("updated_at_lt", "2025-12-22")]);

    let results = spacedevs.extract_bytes().await?;
    serde_json::from_slice::<serde_json::Value>(&results)
        .map(|json| println!("Extracted results: {}", json))?;
    //println!("Extracted results: {:?}", results);

    /*
    let perplexity: RestExtractor =
        RestExtractor::new("https://api.perplexity.ai", "search")
        .with_header("Authorization", &format!("Bearer {}", perp_api_key))
        // Perplexity requires a POST with JSON payload rather than a GET with query params
        .with_method("POST")
        .with_json_body(&serde_json::json!({
            "query": "Latest Developments in AI",
            "max_results": 5,
            "max_tokens_per_page": 2048,
        }));
    
    println!("URL: {}", perplexity.url());

    let results: serde_json::Value = perplexity.extract().await?;
    
    println!("Extracted results: {:?}", results);
    */
    Ok(())
}