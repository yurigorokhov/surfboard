use chrono::{DateTime, FixedOffset, NaiveDateTime, TimeZone, Utc};
use heapless::{String, Vec};
use serde::{Deserialize, Serialize};

use crate::errors::SurfboardLibError;

const MEASUREMENTS_TIDE: usize = 36;
const MEASUREMENTS_WAVE: usize = 24;
const MEASUREMENTS_WIND: usize = 24;

#[derive(Debug, Serialize, Deserialize)]
pub struct SurfReportResponse {
    pub last_updated_utc: i64,
    pub waves: Vec<WaveMeasurement, MEASUREMENTS_WAVE>,
    pub tides: Vec<TideMeasurement, MEASUREMENTS_TIDE>,
    pub wind: Vec<WindMeasurement, MEASUREMENTS_WIND>,
}

impl SurfReportResponse {
    pub fn parse_timestamp_utc(&self) -> Result<DateTime<Utc>, SurfboardLibError> {
        Ok(Utc.timestamp_opt(self.last_updated_utc, 0).unwrap())
    }

    pub fn parse_timestamp_local(&self) -> Result<NaiveDateTime, SurfboardLibError> {
        let offset = FixedOffset::west_opt(-7 * 3600).unwrap();
        let local_time = self.parse_timestamp_utc()?.naive_local();
        Ok(offset.from_local_datetime(&local_time).unwrap().naive_utc())
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WaveMeasurement {
    pub timestamp: i64,
    pub surf: WaveMeasurementSurf,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WaveMeasurementSurf {
    pub min: i32,
    pub max: i32,
    pub plus: bool,
    pub human_relation: String<30>,
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

#[derive(Debug, Serialize, Deserialize)]
pub enum WindDirectionType {
    Onshore,
    Offshore,
    #[serde(rename = "Cross-shore")]
    CrossShore,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WindMeasurement {
    pub timestamp: u64,
    pub utc_offset: i32,
    pub direction: f32,
    pub direction_type: WindDirectionType,
    pub speed: f32,
}
