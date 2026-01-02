

use anduflow::anduflow_core::extract::{Extractor, rest_extractor::RestExtractor};
use anduflow::anduflow_utils::error::ExtractorResult;
use anduflow_utils::logger;
use std::fs::File;
use std::io::Write;
use anduflow_utils::logger::store::{LogStore, ensure_etl_logs_table_exists, };
use dotenv::dotenv;
use std::sync::Arc;
use tracing_subscriber::{FmtSubscriber, fmt::format::FmtSpan};
use tracing::{Level, subscriber};

#[tokio::main]
async fn main() -> ExtractorResult<()> {
    dotenv().ok();
    let as_file = File::create("spacedevs_latest_article.json")?;
    let subscriber =        FmtSubscriber::builder()
        .with_max_level(Level::INFO)
        .with_span_events(FmtSpan::CLOSE)
        .with_writer(as_file)
        .json()
        .with_current_span(true)
        .with_thread_ids(true)
        .with_thread_names(true)
        .with_timer(tracing_subscriber::fmt::time::ChronoLocal::rfc_3339())
        .finish();

    subscriber::set_global_default(subscriber)
        .expect("setting default subscriber failed");
    tracing::info!("Starting RestExtractor example");
    let conn = Arc::new(rusqlite::Connection::open("anduflow_logs.db")?);
    ensure_etl_logs_table_exists(&conn)?;
    tracing::info!("Ensured etl_logs table exists");
    //let _perp_api_key = std::env::var("PERPLEXITY_API_KEY").expect("PERPLEXITY_API_KEY must be set in .env file");
    let mut logger: &mut LogStore = &mut LogStore::new("RestExtractorExample".to_string(), "Extracting from REST API".to_string());
    tracing::info!("Initialized LogStore");
    
    let spacedevs: RestExtractor =
        RestExtractor::new("https://api.spaceflightnewsapi.net/v4", "articles")
        .with_query_param(&[("updated_at_gte", "2025-12-21"), ("ordering", "-updated_at"), ("limit", "1"), ("offset", "0"), ("updated_at_lt", "2025-12-22")]);

    tracing::info!("Created RestExtractor for SpaceDevs API");
    let _results = spacedevs.extract_json::<serde_json::Value>(&mut logger).await?;
    
    



    //serde_json::from_slice::<serde_json::Value>(&results)
    //    .map(|json| println!("Extracted results: {}", json))?;
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