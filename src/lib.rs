// src/lib.rs
use reqwest::header::{HeaderMap, HeaderValue, USER_AGENT};
use serde::{Deserialize, Serialize};
use thiserror::Error;

pub mod models;
pub mod cli;

#[derive(Error, Debug)]
pub enum NseScraperError {
    #[error("HTTP request failed: {0}")]
    RequestError(#[from] reqwest::Error),
    
    #[error("Failed to parse response: {0}")]
    ParseError(#[from] serde_json::Error),
    
    #[error("Invalid symbol: {0}")]
    InvalidSymbol(String),
    
    #[error("API error: {0}")]
    ApiError(String),
    
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
}

pub type Result<T> = std::result::Result<T, NseScraperError>;

pub struct NseScraper {
    client: reqwest::Client,
}

impl NseScraper {
    pub fn new() -> Result<Self> {
        let mut headers = HeaderMap::new();
        headers.insert(USER_AGENT, HeaderValue::from_static("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/91.0.4472.124 Safari/537.36"));
        
        let client = reqwest::Client::builder()
            .default_headers(headers)
            .build()?;
            
        Ok(Self { client })
    }
    
    /// Get quote data for a specific symbol
    pub async fn get_quote(&self, symbol: &str) -> Result<models::QuoteData> {
        let url = format!("https://www.nseindia.com/api/quote-equity?symbol={}", symbol);
        
        let response = self.client
            .get(&url)
            .send()
            .await?;
            
        if !response.status().is_success() {
            return Err(NseScraperError::ApiError(format!(
                "API returned error status: {}", response.status()
            )));
        }
        
        let data = response.json::<models::QuoteData>().await?;
        Ok(data)
    }
    
    /// Get market status
    pub async fn get_market_status(&self) -> Result<models::MarketStatus> {
        let url = "https://www.nseindia.com/api/marketStatus";
        
        let response = self.client
            .get(url)
            .send()
            .await?;
            
        if !response.status().is_success() {
            return Err(NseScraperError::ApiError(format!(
                "API returned error status: {}", response.status()
            )));
        }
        
        let data = response.json::<models::MarketStatus>().await?;
        Ok(data)
    }
    
    /// Get indices data
    pub async fn get_indices(&self) -> Result<models::IndicesData> {
        let url = "https://www.nseindia.com/api/allIndices";
        
        let response = self.client
            .get(url)
            .send()
            .await?;
            
        if !response.status().is_success() {
            return Err(NseScraperError::ApiError(format!(
                "API returned error status: {}", response.status()
            )));
        }
        
        let data = response.json::<models::IndicesData>().await?;
        Ok(data)
    }
    
    /// Get historical data for a symbol
    pub async fn get_historical_data(&self, symbol: &str, series: &str, from_date: &str, to_date: &str) -> Result<models::HistoricalData> {
        let url = format!(
            "https://www.nseindia.com/api/historical/cm/equity?symbol={}&series={}&from={}&to={}",
            symbol, series, from_date, to_date
        );
        
        let response = self.client
            .get(&url)
            .send()
            .await?;
            
        if !response.status().is_success() {
            return Err(NseScraperError::ApiError(format!(
                "API returned error status: {}", response.status()
            )));
        }
        
        let data = response.json::<models::HistoricalData>().await?;
        Ok(data)
    }
}