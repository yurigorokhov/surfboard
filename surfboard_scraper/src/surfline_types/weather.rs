use anyhow::Result;
use serde::{Deserialize, Deserializer, Serialize};

use crate::{http::fetch, surfline_types::common::FetchParams};

#[derive(Debug, Serialize, Deserialize)]
pub struct WeatherResult {
    pub data: WeatherData,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WeatherData {
    pub weather: Vec<WeatherMeasurement>,
}

#[derive(Debug, Serialize, Clone)]
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
    Unknown(String),
}

impl<'de> Deserialize<'de> for WeatherCondition {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        match s.as_str() {
            "NIGHT_MOSTLY_CLOUDY" => Ok(WeatherCondition::NightMostlyCloudy),
            "NIGHT_CLOUDY" => Ok(WeatherCondition::NightCloudy),
            "NIGHT_CLEAR" => Ok(WeatherCondition::NightClear),
            "NIGHT_FOG" => Ok(WeatherCondition::NightFog),
            "MOSTLY_CLOUDY" => Ok(WeatherCondition::MostlyCloudy),
            "MOSTLY_CLEAR" => Ok(WeatherCondition::MostlyClear),
            "MIST" => Ok(WeatherCondition::Mist),
            "CLEAR" => Ok(WeatherCondition::Clear),
            "NIGHT_MOSTLY_CLEAR" => Ok(WeatherCondition::NightMostlyClear),
            "BRIEF_SHOWERS" => Ok(WeatherCondition::BriefShowers),
            "NIGHT_BRIEF_SHOWERS" => Ok(WeatherCondition::NightBriefShowers),
            "NIGHT_MIST" => Ok(WeatherCondition::NightMist),
            "BRIEF_SHOWERS_POSSIBLE" => Ok(WeatherCondition::BriefShowersPossible),
            "NIGHT_DRIZZLE" => Ok(WeatherCondition::NightDrizzle),
            "DRIZZLE" => Ok(WeatherCondition::Drizzle),
            _ => Ok(WeatherCondition::Unknown(s)),
        }
    }
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
