use serde::{Deserialize, Serialize};

use crate::http::fetch;
pub use surfboard_lib::surf_report::WaveMeasurement;

#[derive(Debug, Serialize, Deserialize)]
pub struct WaveResult {
    pub data: WaveData,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WaveData {
    pub wave: Vec<WaveMeasurement>,
}

pub async fn fetch_waves(spot_id: &str) -> Result<WaveResult, Box<dyn std::error::Error>> {
    let url = format!(
        "https://services.surfline.com/kbyg/spots/forecasts/wave?spotId={}&days=2&intervalHours=1",
        spot_id
    );
    fetch(url.as_str()).await
}
