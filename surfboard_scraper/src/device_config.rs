use anyhow::Result;
use embedded_graphics::prelude::Size;
use embedded_graphics_simulator::{OutputSettingsBuilder, SimulatorDisplay};
use epd_waveshare::color::TriColor;
use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::{
    photo::PhotoScreen,
    screen::{Screen, ScreenIdentifier},
    surf_report_24h::data::SurfReport24HData,
};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Configuration {
    pub screens: Vec<ScreenConfiguration>,
    pub screen_saver: Option<ScreenConfiguration>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ScreenConfiguration {
    pub id: String,
    pub key: ScreenIdentifier,
    pub url: String,
    pub params: HashMap<String, Value>,
}

impl ScreenConfiguration {
    pub async fn draw_to_qoi<W>(&self, writer: &mut W) -> Result<()>
    where
        W: std::io::Write + std::io::Seek,
    {
        match self.key {
            ScreenIdentifier::SurfReport24h => {
                let params = SurfReport24HData::parse_params(&self.params)?;
                SurfReport24HData::from_params(&params).await?.draw_to_qoi(writer)?;
                Ok(())
            }
            ScreenIdentifier::Photo => {
                let params = PhotoScreen::parse_params(&self.params)?;
                PhotoScreen::from_params(&params).await?.draw_to_qoi(writer)?;
                Ok(())
            }
        }
    }

    pub async fn draw_to_png(&self, png_path: &str) -> Result<()> {
        let mut display = SimulatorDisplay::<TriColor>::new(Size::new(800, 480));
        match self.key {
            ScreenIdentifier::SurfReport24h => {
                let params = SurfReport24HData::parse_params(&self.params)?;
                SurfReport24HData::from_params(&params).await?.draw(&mut display)?;
            }
            ScreenIdentifier::Photo => {
                let params = PhotoScreen::parse_params(&self.params)?;
                PhotoScreen::from_params(&params).await?.draw(&mut display)?;
            }
        }
        let output_settings = OutputSettingsBuilder::new().scale(1).build();
        let output_image = display.to_grayscale_output_image(&output_settings);
        output_image.save_png(png_path).expect("Failed to save test image");
        Ok(())
    }
}
