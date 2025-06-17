use chrono::{DateTime, FixedOffset, NaiveDateTime, TimeZone, Utc};
// use chrono_tz::Tz;
use heapless::{String, Vec};

use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};

use crate::errors::SurfboardLibError;

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct SurfReportResponse {
    timestamp: String<20>,
    timezone: String<20>,
    tide_predictions: TidePredictions,
}

impl SurfReportResponse {
    fn parse_timestamp_utc(&self) -> Result<DateTime<Utc>, SurfboardLibError> {
        Ok(self.timestamp.parse::<DateTime<Utc>>()?)
    }

    fn parse_timestamp_local(&self) -> Result<NaiveDateTime, SurfboardLibError> {
        let offset = FixedOffset::west_opt(-7 * 3600).unwrap();
        let local_time = self.parse_timestamp_utc()?.naive_local();
        Ok(offset.from_local_datetime(&local_time).unwrap().naive_utc())
    }
}

#[derive(Debug, Clone)]
pub enum DataRetrievalAction {
    SurfReport,
}

#[derive(Default)]
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

    pub fn update_from_surf_report(&mut self, surf_report: SurfReportResponse) -> Result<(), SurfboardLibError> {
        self.set_last_updated(surf_report.parse_timestamp_local()?);
        self.set_tide_predictions(surf_report.tide_predictions);
        Ok(())
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

pub async fn surf_report<'a, T: HttpDataProvider>(client: &'a T) -> Option<SurfReportResponse> {
    let fake_response = "{\"timestamp\":\"2025-06-17T03:37:26Z\",\"timezone\":\"US/Pacific\",\"tide_predictions\":{\"predictions\":[{\"t\":\"2025-06-13 00:00\",\"v\":\"5.009\"},{\"t\":\"2025-06-13 01:00\",\"v\":\"4.048\"},{\"t\":\"2025-06-13 02:00\",\"v\":\"2.804\"},{\"t\":\"2025-06-13 03:00\",\"v\":\"1.475\"},{\"t\":\"2025-06-13 04:00\",\"v\":\"0.286\"},{\"t\":\"2025-06-13 05:00\",\"v\":\"-0.554\"},{\"t\":\"2025-06-13 06:00\",\"v\":\"-0.905\"},{\"t\":\"2025-06-13 07:00\",\"v\":\"-0.729\"},{\"t\":\"2025-06-13 08:00\",\"v\":\"-0.101\"},{\"t\":\"2025-06-13 09:00\",\"v\":\"0.810\"},{\"t\":\"2025-06-13 10:00\",\"v\":\"1.790\"},{\"t\":\"2025-06-13 11:00\",\"v\":\"2.643\"},{\"t\":\"2025-06-13 12:00\",\"v\":\"3.232\"},{\"t\":\"2025-06-13 13:00\",\"v\":\"3.510\"},{\"t\":\"2025-06-13 14:00\",\"v\":\"3.516\"},{\"t\":\"2025-06-13 15:00\",\"v\":\"3.357\"},{\"t\":\"2025-06-13 16:00\",\"v\":\"3.174\"},{\"t\":\"2025-06-13 17:00\",\"v\":\"3.110\"},{\"t\":\"2025-06-13 18:00\",\"v\":\"3.264\"},{\"t\":\"2025-06-13 19:00\",\"v\":\"3.662\"},{\"t\":\"2025-06-13 20:00\",\"v\":\"4.242\"},{\"t\":\"2025-06-13 21:00\",\"v\":\"4.863\"},{\"t\":\"2025-06-13 22:00\",\"v\":\"5.346\"},{\"t\":\"2025-06-13 23:00\",\"v\":\"5.524\"},{\"t\":\"2025-06-14 00:00\",\"v\":\"5.287\"},{\"t\":\"2025-06-14 01:00\",\"v\":\"4.614\"},{\"t\":\"2025-06-14 02:00\",\"v\":\"3.573\"},{\"t\":\"2025-06-14 03:00\",\"v\":\"2.314\"},{\"t\":\"2025-06-14 04:00\",\"v\":\"1.043\"},{\"t\":\"2025-06-14 05:00\",\"v\":\"-0.022\"},{\"t\":\"2025-06-14 06:00\",\"v\":\"-0.692\"}]}}";
    let body = fake_response.as_bytes();
    // client.get_as_json::<TidePredictions>(url).await
    let (data, _remainder) = serde_json_core::from_slice::<SurfReportResponse>(body).unwrap();
    Some(data)
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
        let data = surf_report(&http_provider).await;
        assert!(data.is_some());
        assert!(data.unwrap().tide_predictions.predictions.len() > 0);
    }
}
