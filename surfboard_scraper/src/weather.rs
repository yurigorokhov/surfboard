use serde::{Deserialize, Serialize};

use crate::http::fetch;

#[derive(Debug, Serialize, Deserialize)]
pub struct WeatherResult {
    pub data: WeatherData,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WeatherData {
    pub weather: Vec<WeatherMeasurement>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum WeatherCondition {
    NightMostlyCloudy,
    NightClear,
    MostlyCloudy,
    MostlyClear,
    Mist,
    Clear,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WeatherMeasurement {
    pub timestamp: u32,
    pub utc_offset: i32,
    pub condition: WeatherCondition,
    pub temperature: f32,
}

pub async fn fetch_weather(spot_id: &str) -> Result<WeatherResult, Box<dyn std::error::Error>> {
    let url = format!(
        "https://services.surfline.com/kbyg/spots/forecasts/weather?spotId={}&days=1&intervalHours=4",
        spot_id
    );
    fetch(url.as_str()).await
}
