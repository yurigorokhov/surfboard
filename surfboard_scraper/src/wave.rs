use serde::{Deserialize, Serialize};

use crate::http::fetch;

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
    pub timestamp: u32,
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

pub async fn fetch_waves(spot_id: &str) -> Result<WaveResult, Box<dyn std::error::Error>> {
    let url = format!(
        "https://services.surfline.com/kbyg/spots/forecasts/wave?spotId={}&days=1&intervalHours=1",
        spot_id
    );
    fetch(url.as_str()).await
}
