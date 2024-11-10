use serde::de::DeserializeOwned;
use thiserror::Error;

use crate::models::{DepthInterval, EarningInterval, RunePoolInterval, SwapsInterval};

// Error handling
#[derive(Error, Debug)]
pub(crate) enum ApiError {
    #[error("HTTP request failed: {0}")]
    RequestError(#[from] reqwest::Error),

    #[error("Failed to parse response: {0}")]
    ParseError(#[from] serde_json::Error),

    #[error("Invalid response structure: {0}")]
    InvalidResponse(String),
}

// API client configuration
#[derive(Debug, Clone)]
pub struct ApiConfig {
    pub base_url: String,
    pub timeout_secs: u64,
}

impl Default for ApiConfig {
    fn default() -> Self {
        Self {
            base_url: "https://midgard.ninerealms.com/v2".to_string(),
            timeout_secs: 30,
        }
    }
}

#[derive(Debug, Clone)]
pub struct IntervalParams {
    pub from: i64,
    pub count: i64,
    pub interval: String,
}



pub async fn fetch_interval_data<T: DeserializeOwned>(
    endpoint: &str,
    params: &IntervalParams,
) -> Result<Vec<T>, ApiError> {
    let url = format!(
        "{}/history/{}?interval={}&count={}&from={}",
        ApiConfig::default().base_url, endpoint, params.interval, params.count, params.from
    );

    println!("Fetching data from URL: {}", url);

    let response = reqwest::get(&url)
        .await
        .map_err(ApiError::RequestError)?;

    let json_resp: serde_json::Value = response
        .json()
        .await
        .map_err(ApiError::RequestError)?;

    let intervals = json_resp["intervals"]
        .as_array()
        .ok_or_else(|| ApiError::InvalidResponse("No intervals found in response".to_string()))?;

    serde_json::from_value(serde_json::Value::Array(intervals.to_vec()))
        .map_err(ApiError::ParseError)
}


// Helper functions to make the API calls more ergonomic
pub async fn fetch_depth_data(
    params: &IntervalParams,
    asset: &str,
) -> Result<Vec<DepthInterval>, ApiError> {
    fetch_interval_data(&format!("depths/{}", asset), params).await
}

pub async fn fetch_swaps_data(
    params: &IntervalParams,
) -> Result<Vec<SwapsInterval>, ApiError> {
    fetch_interval_data("swaps", params).await
}

pub async fn fetch_earnings_data(
    params: &IntervalParams,
) -> Result<Vec<EarningInterval>, ApiError> {
    fetch_interval_data("earnings", params).await
}

pub async fn fetch_runepool_data(
    params: &IntervalParams,
) -> Result<Vec<RunePoolInterval>, ApiError> {
    fetch_interval_data("runepool", params).await
}