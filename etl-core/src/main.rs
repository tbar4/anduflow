use etl_core::extract::rest_extractor::RestExtractor;  
use etl_core::extract::Extractor; 

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let extractor = RestExtractor::new("https://api.spaceflightnewsapi.net/v4", "articles")
        .with_header("Accept", "application/json")
        .extract::<serde_json::Value>()
        .await?;

    println!("Extracted data: {:?}", extractor);

    Ok(())
}