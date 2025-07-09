use anyhow::Result;
use serde::{Deserialize, Serialize};

use crate::http::fetch;

#[derive(Debug, Serialize, Deserialize)]
pub struct ConditionsResult {
    pub data: ConditionsData,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ConditionsData {
    pub conditions: Vec<ConditionsMeasurement>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ConditionsMeasurement {
    pub headline: String,
}

pub async fn fetch_conditions(spot_id: &str) -> Result<ConditionsResult> {
    let url = format!(
        "https://services.surfline.com/kbyg/spots/forecasts/conditions?spotId={}&days=1",
        spot_id
    );
    fetch(url.as_str()).await
}
