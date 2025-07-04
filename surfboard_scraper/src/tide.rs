use serde::{Deserialize, Serialize};

use crate::http::fetch;
pub use surfboard_lib::surf_report::TideMeasurement;

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

pub async fn fetch_tides(spot_id: &str) -> Result<TideResult, Box<dyn std::error::Error>> {
    let url = format!(
        "https://services.surfline.com/kbyg/spots/forecasts/tides?spotId={}&days=2",
        spot_id
    );
    fetch(url.as_str()).await
}
