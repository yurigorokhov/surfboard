use anyhow::Result;
use core::fmt::Debug;
use embedded_graphics::prelude::*;
use embedded_graphics_simulator::{OutputSettingsBuilder, SimulatorDisplay};
use epd_waveshare::color::TriColor;
use serde_json::Value;
use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::{message::draw::draw, screen::Screen};

#[derive(Debug, Serialize, Deserialize)]
pub struct MessageParams {
    pub message: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MessageData {
    pub message: String,
}

impl Screen<MessageParams> for MessageData {
    async fn from_params(params: &MessageParams) -> Result<Box<Self>> {
        Ok(Box::new(MessageData {
            message: params.message.clone(),
        }))
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

    fn parse_params(params: &HashMap<String, Value>) -> Result<MessageParams> {
        let message = params
            .get("message")
            .and_then(|v| v.as_str())
            .map(|s| s.to_string());
        Ok(MessageParams { message: message.unwrap() })
    }

    fn draw<D, E>(&self, target: &mut D) -> Result<(), E>
    where
        E: Debug,
        D: DrawTarget<Color = TriColor, Error = E>,
    {
        draw(target, self)
    }
}
