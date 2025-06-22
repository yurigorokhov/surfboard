use serde::{Deserialize, Serialize};
pub use surfboard_lib::surf_report::WeatherMeasurement;

use crate::http::fetch;

#[derive(Debug, Serialize, Deserialize)]
pub struct WeatherResult {
    pub data: WeatherData,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WeatherData {
    pub weather: Vec<WeatherMeasurement>,
}

pub async fn fetch_weather(spot_id: &str) -> Result<WeatherResult, Box<dyn std::error::Error>> {
    let url = format!(
        "https://services.surfline.com/kbyg/spots/forecasts/weather?spotId={}&days=2&intervalHours=1",
        spot_id
    );
    fetch(url.as_str()).await
}
