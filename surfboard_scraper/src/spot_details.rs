use serde::{Deserialize, Serialize};
pub use surfboard_lib::surf_report::SpotDetails;

use crate::http::fetch;

#[derive(Debug, Serialize, Deserialize)]
pub struct SpotDetailsResult {
    pub spot: SpotDetails,
}

pub async fn fetch_spot_details(spot_id: &str) -> Result<SpotDetailsResult, Box<dyn std::error::Error>> {
    let url = format!("https://services.surfline.com/kbyg/spots/details?spotId={}", spot_id);
    fetch(url.as_str()).await
}
