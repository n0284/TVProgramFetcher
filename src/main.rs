use tvprogramfetcher::run_batch;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>>{
    run_batch().await?;
    Ok(())
}