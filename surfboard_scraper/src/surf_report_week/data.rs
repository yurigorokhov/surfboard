use anyhow::Result;
use chrono::prelude::*;
use core::fmt::Debug;
use embedded_graphics::prelude::*;
use embedded_graphics_simulator::{OutputSettingsBuilder, SimulatorDisplay};
use epd_waveshare::color::TriColor;
use serde_json::Value;
use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::{
    screen::Screen,
    surf_report_week::draw::draw,
    surfline_types::{
        common::FetchParams,
        spot_details::{SpotDetails, SpotDetailsResult, fetch_spot_details},
        tide::{TideMeasurement, TideResult, fetch_tides},
        wave::{WaveMeasurement, WaveResult, fetch_waves},
        weather::{WeatherMeasurement, WeatherResult, fetch_weather},
        wind::{WindMeasurement, WindResult, fetch_wind},
    },
};

#[derive(Debug, Serialize, Deserialize)]
pub struct SurfReportWeekParams {
    spot_id: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SurfReportWeekData {
    pub last_updated_utc: i64,
    pub waves: Vec<WaveMeasurement>,
    pub tides: Vec<TideMeasurement>,
    pub weather: Vec<WeatherMeasurement>,
    pub wind: Vec<WindMeasurement>,
    pub spot_details: SpotDetails,
}

impl Screen<SurfReportWeekParams> for SurfReportWeekData {
    async fn from_params(params: &SurfReportWeekParams) -> Result<Box<Self>> {
        let spot_id = params.spot_id.as_str();
        let fetch_params = FetchParams {
            days: 7,
            interval_hours: 6,
        };
        Ok(Box::new(SurfReportWeekData::new_from_results(
            fetch_waves(spot_id, Some(fetch_params.clone())).await?,
            fetch_tides(spot_id, Some(fetch_params.clone())).await?,
            fetch_weather(spot_id, Some(fetch_params.clone())).await?,
            fetch_wind(spot_id, Some(fetch_params.clone())).await?,
            fetch_spot_details(spot_id).await?,
        )))
    }

    fn draw_to_qoi<W>(&self, writer: &mut W) -> Result<()>
    where
        W: std::io::Write + std::io::Seek,
    {
        let mut display = SimulatorDisplay::<TriColor>::new(Size::new(800, 480));
        self.draw(&mut display)?;
        let output_settings = OutputSettingsBuilder::new().scale(1).build();
        let output_image = display.to_rgb_output_image(&output_settings);
        let image_buffer = output_image.as_image_buffer();
        image_buffer.write_to(writer, image::ImageFormat::Qoi).unwrap();
        Ok(())
    }

    fn parse_params(params: &HashMap<String, Value>) -> Result<SurfReportWeekParams> {
        let spot_id = params.get("spot_id").unwrap().as_str().unwrap();
        Ok(SurfReportWeekParams {
            spot_id: spot_id.into(),
        })
    }

    fn draw<D, E>(&self, target: &mut D) -> Result<(), E>
    where
        E: Debug,
        D: DrawTarget<Color = TriColor, Error = E>,
    {
        draw(target, self)
    }
}

impl SurfReportWeekData {
    pub fn new_from_results(
        wave_result: WaveResult,
        tide_result: TideResult,
        weather_result: WeatherResult,
        wind_result: WindResult,
        spot_details_result: SpotDetailsResult,
    ) -> Self {
        let now = Utc::now();
        SurfReportWeekData {
            last_updated_utc: now.timestamp(),
            waves: wave_result.data.wave.into_iter().collect(),
            tides: tide_result.data.tides.into_iter().collect(),
            weather: weather_result.data.weather.into_iter().step_by(3).collect(),
            wind: wind_result.data.wind.into_iter().collect(),
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
