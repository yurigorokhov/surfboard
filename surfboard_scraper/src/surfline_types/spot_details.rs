use anyhow::Result;
use serde::{Deserialize, Serialize};

use crate::http::fetch;

#[derive(Debug, Serialize, Deserialize)]
pub struct SpotDetailsResult {
    pub spot: SpotDetails,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SpotDetails {
    pub name: String,
}

pub async fn fetch_spot_details(spot_id: &str) -> Result<SpotDetailsResult> {
    let url = format!("https://services.surfline.com/kbyg/spots/details?spotId={}", spot_id);
    fetch(url.as_str()).await
}
