use chrono::prelude::*;

use serde::{Deserialize, Serialize};
use surfboard_lib::surf_report::{MEASUREMENTS_TIDE, MEASUREMENTS_WAVE, MEASUREMENTS_WEATHER, MEASUREMENTS_WIND};

use crate::{
    conditions::{ConditionsMeasurement, ConditionsResult},
    spot_details::{SpotDetails, SpotDetailsResult},
    tide::{TideMeasurement, TideResult},
    wave::{WaveMeasurement, WaveResult},
    weather::{WeatherMeasurement, WeatherResult},
    wind::{WindMeasurement, WindResult},
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
}
