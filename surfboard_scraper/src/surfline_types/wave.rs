use anyhow::Result;
use serde::{Deserialize, Serialize};

use crate::{http::fetch, surfline_types::common::FetchParams};

#[derive(Debug, Serialize, Deserialize)]
pub struct WaveResult {
    pub data: WaveData,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WaveData {
    pub wave: Vec<WaveMeasurement>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WaveMeasurement {
    pub timestamp: i64,
    pub surf: WaveMeasurementSurf,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WaveMeasurementSurf {
    pub min: i32,
    pub max: i32,
    pub plus: bool,
    pub human_relation: String,
}

pub async fn fetch_waves(spot_id: &str, params: Option<FetchParams>) -> Result<WaveResult> {
    let params = params.unwrap_or_default();
    let url = format!(
        "https://services.surfline.com/kbyg/spots/forecasts/wave?spotId={}&days={}&intervalHours={}",
        spot_id, params.days, params.interval_hours,
    );
    fetch(url.as_str()).await
}
