use anyhow::Result;
use serde::{Deserialize, Serialize};

use crate::{http::fetch, surfline_types::common::FetchParams};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TideResultAssociatedData {
    pub utc_offset: i32,
    pub tide_location: TideResultAssociatedDataLocation,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TideResultAssociatedDataLocation {
    pub name: String,
    pub min: f32,
    pub max: f32,
    pub mean: f32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TideResult {
    pub data: Tides,
    pub associated: TideResultAssociatedData,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Tides {
    pub tides: Vec<TideMeasurement>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub enum TideType {
    NORMAL,
    HIGH,
    LOW,
}

impl TideType {
    pub fn is_high_low(&self) -> bool {
        self == &TideType::HIGH || self == &TideType::LOW
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TideMeasurement {
    pub height: f32,
    pub timestamp: i64,
    pub r#type: TideType,
    pub utc_offset: i32,
}

pub async fn fetch_tides(spot_id: &str, params: Option<FetchParams>) -> Result<TideResult> {
    let params = params.unwrap_or_default();
    let url = format!(
        "https://services.surfline.com/kbyg/spots/forecasts/tides?spotId={}&days={}",
        spot_id, params.days
    );
    fetch(url.as_str()).await
}
