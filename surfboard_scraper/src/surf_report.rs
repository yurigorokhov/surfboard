use chrono::prelude::*;

use serde::{Deserialize, Serialize};

use crate::{
    tide::{TideMeasurement, TideResult},
    wave::{WaveMeasurement, WaveResult},
};

const HOURLY_MEASUREMENT_COUNT: usize = 36;

#[derive(Debug, Serialize, Deserialize)]
pub struct SurfReport {
    pub last_updated_utc: i64,
    pub wave_data: Vec<WaveMeasurement>,
    pub tides: Vec<TideMeasurement>,
}

impl SurfReport {
    pub fn new_from_results(wave_result: WaveResult, tide_result: TideResult) -> Self {
        let now = Utc::now();
        SurfReport {
            last_updated_utc: now.timestamp(),
            wave_data: wave_result
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
        }
    }
}
