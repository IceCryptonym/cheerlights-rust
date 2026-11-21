use serde::Deserialize;
use thiserror::Error;

use crate::types::Color;

const LATEST_COLOR_URL: &str = "https://api.thingspeak.com/channels/1417/feeds/last.json";
const COLOR_HISTORY_URL: &str = "https://api.thingspeak.com/channels/1417/feed.json";

#[derive(Debug, Deserialize)]
pub struct ColorUpdate {
    pub created_at: String,
    pub entry_id: i32,
    #[serde(rename = "field1")]
    pub color: Color,
}

#[derive(Debug, Deserialize)]
pub struct ColorHistory {
    #[serde(rename = "feeds")]
    pub updates: Vec<ColorUpdate>,
}

#[derive(Default)]
pub struct Client {
    http: reqwest::Client,
}

#[derive(Debug, Error)]
pub enum FetchError {
    #[error("Get request timed out")]
    Timeout,
    #[error("Failed to parse JSON")]
    Parse,
}

impl Client {
    pub async fn fetch_latest(&self) -> Result<ColorUpdate, FetchError> {
        self.http
            .get(LATEST_COLOR_URL)
            .send()
            .await
            .map_err(|_| FetchError::Timeout)?
            .json()
            .await
            .map_err(|_| FetchError::Parse)
    }

    pub async fn fetch_history(&self) -> Result<ColorHistory, FetchError> {
        self.http
            .get(COLOR_HISTORY_URL)
            .send()
            .await
            .map_err(|_| FetchError::Timeout)?
            .json()
            .await
            .map_err(|_| FetchError::Parse)
    }
}
