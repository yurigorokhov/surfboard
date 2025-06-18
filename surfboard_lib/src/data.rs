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

#[trait_variant::make(HttpService: Send)]
pub trait HttpDataProvider {
    async fn get_as_json<'a, DataType: DeserializeOwned>(&'a self, url: &'a str) -> Option<DataType>;
}

/****** Tide predictions ******/
pub const TIDE_PREDICTIONS_LEN: usize = 36;

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
    let fake_response = "{\"last_updated_utc\":1750217245,\"wave_data\":[{\"timestamp\":1750143600,\"surf\":{\"min\":3,\"max\":4,\"plus\":true,\"humanRelation\":\"Waist to shoulder\"}},{\"timestamp\":1750147200,\"surf\":{\"min\":3,\"max\":4,\"plus\":true,\"humanRelation\":\"Waist to shoulder\"}},{\"timestamp\":1750150800,\"surf\":{\"min\":3,\"max\":4,\"plus\":false,\"humanRelation\":\"Waist to chest\"}},{\"timestamp\":1750154400,\"surf\":{\"min\":3,\"max\":4,\"plus\":false,\"humanRelation\":\"Waist to chest\"}},{\"timestamp\":1750158000,\"surf\":{\"min\":3,\"max\":4,\"plus\":false,\"humanRelation\":\"Waist to chest\"}},{\"timestamp\":1750161600,\"surf\":{\"min\":3,\"max\":4,\"plus\":false,\"humanRelation\":\"Waist to chest\"}},{\"timestamp\":1750165200,\"surf\":{\"min\":3,\"max\":4,\"plus\":false,\"humanRelation\":\"Waist to chest\"}},{\"timestamp\":1750168800,\"surf\":{\"min\":2,\"max\":3,\"plus\":true,\"humanRelation\":\"Thigh to stomach\"}},{\"timestamp\":1750172400,\"surf\":{\"min\":2,\"max\":3,\"plus\":true,\"humanRelation\":\"Thigh to stomach\"}},{\"timestamp\":1750176000,\"surf\":{\"min\":2,\"max\":3,\"plus\":true,\"humanRelation\":\"Thigh to stomach\"}},{\"timestamp\":1750179600,\"surf\":{\"min\":2,\"max\":3,\"plus\":false,\"humanRelation\":\"Thigh to waist\"}},{\"timestamp\":1750183200,\"surf\":{\"min\":2,\"max\":3,\"plus\":false,\"humanRelation\":\"Thigh to waist\"}},{\"timestamp\":1750186800,\"surf\":{\"min\":2,\"max\":3,\"plus\":false,\"humanRelation\":\"Thigh to waist\"}},{\"timestamp\":1750190400,\"surf\":{\"min\":2,\"max\":3,\"plus\":false,\"humanRelation\":\"Thigh to waist\"}},{\"timestamp\":1750194000,\"surf\":{\"min\":2,\"max\":3,\"plus\":false,\"humanRelation\":\"Thigh to waist\"}},{\"timestamp\":1750197600,\"surf\":{\"min\":2,\"max\":3,\"plus\":false,\"humanRelation\":\"Thigh to waist\"}},{\"timestamp\":1750201200,\"surf\":{\"min\":2,\"max\":3,\"plus\":true,\"humanRelation\":\"Thigh to stomach\"}},{\"timestamp\":1750204800,\"surf\":{\"min\":2,\"max\":3,\"plus\":true,\"humanRelation\":\"Thigh to stomach\"}},{\"timestamp\":1750208400,\"surf\":{\"min\":2,\"max\":3,\"plus\":true,\"humanRelation\":\"Thigh to stomach\"}},{\"timestamp\":1750212000,\"surf\":{\"min\":2,\"max\":3,\"plus\":true,\"humanRelation\":\"Thigh to stomach\"}},{\"timestamp\":1750215600,\"surf\":{\"min\":2,\"max\":3,\"plus\":true,\"humanRelation\":\"Thigh to stomach\"}},{\"timestamp\":1750219200,\"surf\":{\"min\":2,\"max\":3,\"plus\":true,\"humanRelation\":\"Thigh to stomach\"}},{\"timestamp\":1750222800,\"surf\":{\"min\":2,\"max\":3,\"plus\":true,\"humanRelation\":\"Thigh to stomach\"}},{\"timestamp\":1750226400,\"surf\":{\"min\":2,\"max\":3,\"plus\":false,\"humanRelation\":\"Thigh to waist\"}}],\"tides\":[{\"height\":3.51,\"timestamp\":1750143600,\"type\":\"NORMAL\",\"utcOffset\":-7},{\"height\":3.88,\"timestamp\":1750147200,\"type\":\"NORMAL\",\"utcOffset\":-7},{\"height\":4.15,\"timestamp\":1750150800,\"type\":\"NORMAL\",\"utcOffset\":-7},{\"height\":4.2,\"timestamp\":1750153090,\"type\":\"HIGH\",\"utcOffset\":-7},{\"height\":4.18,\"timestamp\":1750154400,\"type\":\"NORMAL\",\"utcOffset\":-7},{\"height\":3.87,\"timestamp\":1750158000,\"type\":\"NORMAL\",\"utcOffset\":-7},{\"height\":3.19,\"timestamp\":1750161600,\"type\":\"NORMAL\",\"utcOffset\":-7},{\"height\":2.26,\"timestamp\":1750165200,\"type\":\"NORMAL\",\"utcOffset\":-7},{\"height\":1.25,\"timestamp\":1750168800,\"type\":\"NORMAL\",\"utcOffset\":-7},{\"height\":0.42,\"timestamp\":1750172400,\"type\":\"NORMAL\",\"utcOffset\":-7},{\"height\":-0.06,\"timestamp\":1750176000,\"type\":\"NORMAL\",\"utcOffset\":-7},{\"height\":-0.13,\"timestamp\":1750178096,\"type\":\"LOW\",\"utcOffset\":-7},{\"height\":-0.09,\"timestamp\":1750179600,\"type\":\"NORMAL\",\"utcOffset\":-7},{\"height\":0.35,\"timestamp\":1750183200,\"type\":\"NORMAL\",\"utcOffset\":-7},{\"height\":1.13,\"timestamp\":1750186800,\"type\":\"NORMAL\",\"utcOffset\":-7},{\"height\":2.09,\"timestamp\":1750190400,\"type\":\"NORMAL\",\"utcOffset\":-7},{\"height\":3.07,\"timestamp\":1750194000,\"type\":\"NORMAL\",\"utcOffset\":-7},{\"height\":3.9,\"timestamp\":1750197600,\"type\":\"NORMAL\",\"utcOffset\":-7},{\"height\":4.41,\"timestamp\":1750201200,\"type\":\"NORMAL\",\"utcOffset\":-7},{\"height\":4.53,\"timestamp\":1750203658,\"type\":\"HIGH\",\"utcOffset\":-7},{\"height\":4.5,\"timestamp\":1750204800,\"type\":\"NORMAL\",\"utcOffset\":-7},{\"height\":4.24,\"timestamp\":1750208400,\"type\":\"NORMAL\",\"utcOffset\":-7},{\"height\":3.76,\"timestamp\":1750212000,\"type\":\"NORMAL\",\"utcOffset\":-7},{\"height\":3.19,\"timestamp\":1750215600,\"type\":\"NORMAL\",\"utcOffset\":-7},{\"height\":2.71,\"timestamp\":1750219200,\"type\":\"NORMAL\",\"utcOffset\":-7},{\"height\":2.44,\"timestamp\":1750222800,\"type\":\"NORMAL\",\"utcOffset\":-7},{\"height\":2.4,\"timestamp\":1750225050,\"type\":\"LOW\",\"utcOffset\":-7},{\"height\":2.41,\"timestamp\":1750226400,\"type\":\"NORMAL\",\"utcOffset\":-7},{\"height\":2.61,\"timestamp\":1750230000,\"type\":\"NORMAL\",\"utcOffset\":-7},{\"height\":2.95,\"timestamp\":1750233600,\"type\":\"NORMAL\",\"utcOffset\":-7},{\"height\":3.33,\"timestamp\":1750237200,\"type\":\"NORMAL\",\"utcOffset\":-7},{\"height\":3.63,\"timestamp\":1750240800,\"type\":\"NORMAL\",\"utcOffset\":-7},{\"height\":3.71,\"timestamp\":1750243741,\"type\":\"HIGH\",\"utcOffset\":-7},{\"height\":3.71,\"timestamp\":1750244400,\"type\":\"NORMAL\",\"utcOffset\":-7},{\"height\":3.5,\"timestamp\":1750248000,\"type\":\"NORMAL\",\"utcOffset\":-7},{\"height\":2.97,\"timestamp\":1750251600,\"type\":\"NORMAL\",\"utcOffset\":-7}]}";
    let body = fake_response.as_bytes();
    // client.get_as_json::<TidePredictions>(url).await
    let (data, _remainder) = serde_json_core::from_slice::<SurfReportResponse>(body).unwrap();
    Some(data)
}

//TODO: fetch time: https://github.com/1-rafael-1/pi-pico-alarmclock-rust/blob/main/src/task/time_updater.rs#L290
