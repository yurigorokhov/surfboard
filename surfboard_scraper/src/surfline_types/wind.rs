use anyhow::Result;
use serde::{Deserialize, Serialize};

use crate::{http::fetch, surfline_types::common::FetchParams};

#[derive(Debug, Serialize, Deserialize)]
pub struct WindResult {
    pub data: WindData,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WindData {
    pub wind: Vec<WindMeasurement>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Hash, Clone, Copy)]
pub enum WindDirectionType {
    Onshore,
    Offshore,
    #[serde(rename = "Cross-shore")]
    CrossShore,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WindMeasurement {
    pub timestamp: i64,
    pub utc_offset: i32,
    pub direction: f32,
    pub direction_type: WindDirectionType,
    pub speed: f32,
}

pub async fn fetch_wind(spot_id: &str, params: Option<FetchParams>) -> Result<WindResult> {
    let params = params.unwrap_or_default();
    let url = format!(
        "https://services.surfline.com/kbyg/spots/forecasts/wind?spotId={}&days={}&intervalHours={}",
        spot_id, params.days, params.interval_hours
    );
    fetch(url.as_str()).await
}
