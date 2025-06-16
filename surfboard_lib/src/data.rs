use chrono::NaiveDateTime;
use heapless::{String, Vec};

use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone)]
pub enum DataRetrievalAction {
    TideChart,
}

pub struct ProgramState {
    pub tide_predictions: Option<TidePredictions>,
    pub last_updated: Option<NaiveDateTime>,
}

impl ProgramState {
    pub fn set_tide_predictions(&mut self, predictions: TidePredictions) {
        self.tide_predictions = Some(predictions);
    }

    pub fn set_last_updated(&mut self, last_updated: NaiveDateTime) {
        self.last_updated = Some(last_updated);
    }
}

#[trait_variant::make(HttpService: Send)]
pub trait HttpDataProvider {
    async fn get_as_json<'a, DataType: DeserializeOwned>(&'a self, url: &'a str) -> Option<DataType>;
}

/****** Tide predictions ******/
pub const TIDE_PREDICTIONS_LEN: usize = 32;

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct TidePredictionsDataPoint {
    pub t: String<16>,
    pub v: String<8>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct TidePredictions {
    pub predictions: Vec<TidePredictionsDataPoint, TIDE_PREDICTIONS_LEN>,
}

pub async fn tide_data<'a, T: HttpDataProvider>(client: &'a T) -> Option<TidePredictions> {
    let url = "https://api.tidesandcurrents.noaa.gov/api/prod/datagetter?begin_date=20250613&range=30&station=9413450&product=predictions&datum=STND&time_zone=lst&interval=h&units=english&format=json";
    client.get_as_json::<TidePredictions>(url).await
}

//TODO: fetch time: https://github.com/1-rafael-1/pi-pico-alarmclock-rust/blob/main/src/task/time_updater.rs#L290

#[cfg(test)]
mod tests {
    use super::*;
    use reqwest;

    struct TestDataProvider {}

    impl HttpDataProvider for TestDataProvider {
        async fn get_as_json<'a, DataType: DeserializeOwned>(&'a self, url: &'a str) -> Option<DataType> {
            let response_str = reqwest::get(url).await.ok()?.text().await.ok()?;
            let response_bytes = response_str.as_bytes();
            let mut buffer = [0u8; 4096];
            buffer[0..response_bytes.len()].copy_from_slice(response_bytes);
            let (data, _remainder) = serde_json_core::from_slice::<DataType>(&buffer[0..response_bytes.len()]).unwrap();
            Some(data)
        }
    }

    #[tokio::test]
    async fn test_get_tide_data() {
        let http_provider = TestDataProvider {};
        let data = tide_data(&http_provider).await;
        assert!(data.is_some());
        assert!(data.unwrap().predictions.len() > 0);
    }
}
