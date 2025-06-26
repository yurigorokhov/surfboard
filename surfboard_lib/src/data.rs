use heapless::{String, Vec};

use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};

use crate::surf_report::SurfReportResponse;

#[derive(Debug, Clone)]
pub enum DataRetrievalAction {
    SurfReport,
}

#[derive(Default)]
pub struct ProgramState {
    pub surf_report: Option<SurfReportResponse>,
}

impl ProgramState {
    pub fn update_surf_report(&mut self, surf_report: SurfReportResponse) {
        self.surf_report = Some(surf_report);
    }
}

// #[trait_variant::make(HttpService: Send)]
pub trait HttpDataProvider<DataType: DeserializeOwned> {
    async fn get_as_json<'a>(&'a self, url: &'a str) -> Option<DataType>;
}

/****** Tide predictions ******/
pub const TIDE_PREDICTIONS_LEN: usize = 36;
pub const WAVE_PREDICTIONS_LEN: usize = 36;

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct TidePredictionsDataPoint {
    pub t: String<16>,
    pub v: String<8>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct TidePredictions {
    pub predictions: Vec<TidePredictionsDataPoint, TIDE_PREDICTIONS_LEN>,
}

pub async fn surf_report<'a, T: HttpDataProvider<SurfReportResponse>>(client: &'a T) -> Option<SurfReportResponse> {
    let url = "https://yurig-public.s3.us-east-1.amazonaws.com/surf_report.json";
    client.get_as_json(url).await
}
