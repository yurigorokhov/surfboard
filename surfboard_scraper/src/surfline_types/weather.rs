use anyhow::Result;
use serde::{Deserialize, Serialize};

use crate::{http::fetch, surfline_types::common::FetchParams};

#[derive(Debug, Serialize, Deserialize)]
pub struct WeatherResult {
    pub data: WeatherData,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WeatherData {
    pub weather: Vec<WeatherMeasurement>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum WeatherCondition {
    NightMostlyCloudy,
    NightCloudy,
    NightClear,
    NightFog,
    MostlyCloudy,
    MostlyClear,
    Mist,
    Clear,
    NightMostlyClear,
    BriefShowers,
    NightBriefShowers,
    NightMist,
    BriefShowersPossible,
    NightDrizzle,
    Drizzle,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WeatherMeasurement {
    pub timestamp: i64,
    pub utc_offset: i32,
    pub condition: WeatherCondition,
    pub temperature: f32,
}

pub async fn fetch_weather(spot_id: &str, params: Option<FetchParams>) -> Result<WeatherResult> {
    let params = params.unwrap_or_default();
    let url = format!(
        "https://services.surfline.com/kbyg/spots/forecasts/weather?spotId={}&days={}&intervalHours={}",
        spot_id, params.days, params.interval_hours
    );
    fetch(url.as_str()).await
}
