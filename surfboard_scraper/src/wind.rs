use serde::{Deserialize, Serialize};

use crate::http::fetch;

#[derive(Debug, Serialize, Deserialize)]
pub struct WindResult {
    pub data: WindData,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WindData {
    pub wind: Vec<WindMeasurement>,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum WindDirectionType {
    Onshore,
    Offshore,
    #[serde(rename = "Cross-shore")]
    CrossShore,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WindMeasurement {
    pub timestamp: u64,
    pub utc_offset: i32,
    pub direction: f32,
    pub direction_type: WindDirectionType,
    pub speed: f32,
}

pub async fn fetch_wind(spot_id: &str) -> Result<WindResult, Box<dyn std::error::Error>> {
    let url = format!(
        "https://services.surfline.com/kbyg/spots/forecasts/wind?spotId={}&days=1&intervalHours=1",
        spot_id
    );
    fetch(url.as_str()).await
}
