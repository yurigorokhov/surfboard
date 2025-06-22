use serde::{Deserialize, Serialize};
pub use surfboard_lib::surf_report::ConditionsMeasurement;

use crate::http::fetch;

#[derive(Debug, Serialize, Deserialize)]
pub struct ConditionsResult {
    pub data: ConditionsData,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ConditionsData {
    pub conditions: Vec<ConditionsMeasurement>,
}

pub async fn fetch_conditions(spot_id: &str) -> Result<ConditionsResult, Box<dyn std::error::Error>> {
    let url = format!(
        "https://services.surfline.com/kbyg/spots/forecasts/conditions?spotId={}&days=1",
        spot_id
    );
    fetch(url.as_str()).await
}
