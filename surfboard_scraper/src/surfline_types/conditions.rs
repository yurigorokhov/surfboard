use anyhow::Result;
use serde::{Deserialize, Serialize};

use crate::{http::fetch, surfline_types::common::FetchParams};

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

pub async fn fetch_conditions(spot_id: &str, params: Option<FetchParams>) -> Result<ConditionsResult> {
    let params = params.unwrap_or(FetchParams { days: 1, interval_hours: 1 });
    let url = format!(
        "https://services.surfline.com/kbyg/spots/forecasts/conditions?spotId={}&days={}",
        spot_id, params.days
    );
    fetch(url.as_str()).await
}
