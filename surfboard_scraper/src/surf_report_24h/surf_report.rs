use anyhow::Result;
use chrono::prelude::*;

use serde::{Deserialize, Serialize};

pub const MEASUREMENTS_TIDE: usize = 36;
pub const MEASUREMENTS_WAVE: usize = 10;
pub const MEASUREMENTS_WIND: usize = 10;
pub const MEASUREMENTS_WEATHER: usize = 10;

use crate::{
    surf_report_24h::conditions::{ConditionsMeasurement, ConditionsResult},
    surf_report_24h::spot_details::{SpotDetails, SpotDetailsResult},
    surf_report_24h::tide::{TideMeasurement, TideResult},
    surf_report_24h::wave::{WaveMeasurement, WaveResult},
    surf_report_24h::weather::{WeatherMeasurement, WeatherResult},
    surf_report_24h::wind::{WindMeasurement, WindResult},
};

#[derive(Debug, Serialize, Deserialize)]
pub struct SurfReport {
    pub last_updated_utc: i64,
    pub waves: Vec<WaveMeasurement>,
    pub tides: Vec<TideMeasurement>,
    pub weather: Vec<WeatherMeasurement>,
    pub wind: Vec<WindMeasurement>,
    pub conditions: ConditionsMeasurement,
    pub spot_details: SpotDetails,
}

impl SurfReport {
    pub fn new_from_results(
        wave_result: WaveResult,
        tide_result: TideResult,
        weather_result: WeatherResult,
        wind_result: WindResult,
        conditions_result: ConditionsResult,
        spot_details_result: SpotDetailsResult,
    ) -> Self {
        let now = Utc::now();
        SurfReport {
            last_updated_utc: now.timestamp(),
            waves: wave_result
                .data
                .wave
                .into_iter()
                .skip(6)
                .step_by(3)
                .take(MEASUREMENTS_WAVE)
                .collect(),
            tides: tide_result
                .data
                .tides
                .into_iter()
                .skip(6)
                .take(MEASUREMENTS_TIDE)
                .collect(),
            weather: weather_result
                .data
                .weather
                .into_iter()
                .skip(6)
                .step_by(3)
                .take(MEASUREMENTS_WEATHER)
                .collect(),
            wind: wind_result
                .data
                .wind
                .into_iter()
                .skip(6)
                .step_by(3)
                .take(MEASUREMENTS_WIND)
                .collect(),
            conditions: conditions_result.data.conditions.into_iter().next().unwrap(),
            spot_details: spot_details_result.spot,
        }
    }

    pub fn parse_timestamp_utc(&self) -> Result<DateTime<Utc>> {
        Ok(Utc.timestamp_opt(self.last_updated_utc, 0).unwrap())
    }

    pub fn parse_timestamp_local(&self) -> Result<NaiveDateTime> {
        let offset = FixedOffset::west_opt(-7 * 3600).unwrap();
        let local_time = self.parse_timestamp_utc()?.naive_local();
        Ok(offset.from_local_datetime(&local_time).unwrap().naive_utc())
    }
}
