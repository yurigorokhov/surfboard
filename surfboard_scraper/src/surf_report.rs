use serde::{Deserialize, Serialize};

use crate::{
    tide::{self, TideResult, Tides},
    wave::{WaveData, WaveResult},
};

#[derive(Debug, Serialize, Deserialize)]
pub struct SurfReport {
    pub wave_data: WaveData,
    pub tides: Tides,
}

impl SurfReport {
    pub fn new_from_results(wave_result: WaveResult, tide_result: TideResult) -> Self {
        SurfReport {
            wave_data: wave_result.data,
            tides: tide_result.data,
        }
    }
}
