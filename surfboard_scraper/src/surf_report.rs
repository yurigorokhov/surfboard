use chrono::prelude::*;

use serde::{Deserialize, Serialize};

use crate::{
    conditions::{ConditionsMeasurement, ConditionsResult},
    spot_details::{SpotDetails, SpotDetailsResult},
    tide::{TideMeasurement, TideResult},
    wave::{WaveMeasurement, WaveResult},
    weather::{WeatherMeasurement, WeatherResult},
    wind::{WindMeasurement, WindResult},
};

const HOURLY_MEASUREMENT_COUNT: usize = 36;

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
                .take(HOURLY_MEASUREMENT_COUNT)
                .collect(),
            tides: tide_result
                .data
                .tides
                .into_iter()
                .take(HOURLY_MEASUREMENT_COUNT)
                .collect(),
            weather: weather_result
                .data
                .weather
                .into_iter()
                .take(HOURLY_MEASUREMENT_COUNT)
                .collect(),
            wind: wind_result
                .data
                .wind
                .into_iter()
                .take(HOURLY_MEASUREMENT_COUNT)
                .collect(),
            conditions: conditions_result.data.conditions.into_iter().next().unwrap(),
            spot_details: spot_details_result.spot,
        }
    }
}
