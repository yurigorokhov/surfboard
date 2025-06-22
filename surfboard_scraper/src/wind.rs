use serde::{Deserialize, Serialize};

use crate::http::fetch;
pub use surfboard_lib::surf_report::WindMeasurement;

#[derive(Debug, Serialize, Deserialize)]
pub struct WindResult {
    pub data: WindData,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WindData {
    pub wind: Vec<WindMeasurement>,
}

pub async fn fetch_wind(spot_id: &str) -> Result<WindResult, Box<dyn std::error::Error>> {
    let url = format!(
        "https://services.surfline.com/kbyg/spots/forecasts/wind?spotId={}&days=2&intervalHours=1",
        spot_id
    );
    fetch(url.as_str()).await
}
