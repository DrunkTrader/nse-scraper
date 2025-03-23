// src/main.rs
use nse_scraper::cli::NseCli;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = NseCli::new()?;
    cli.run().await?;
    Ok(())
}