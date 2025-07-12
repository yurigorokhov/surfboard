use anyhow::Result;
use core::fmt::Debug;
use embedded_graphics::prelude::*;
use embedded_graphics_simulator::{OutputSettingsBuilder, SimulatorDisplay};
use epd_waveshare::color::TriColor;
use image::imageops::{self, FilterType};
use image::{Luma, open};
use serde::{Deserialize, Serialize};

use crate::screen::Screen;

#[derive(Debug, Serialize, Deserialize)]
pub struct ScreenSaverParams {
    path: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ScreenSaverScreen {
    path: String,
}

impl Screen<ScreenSaverParams> for ScreenSaverScreen {
    fn parse_params(
        params: &std::collections::HashMap<String, serde_json::Value>,
    ) -> anyhow::Result<ScreenSaverParams> {
        Ok(ScreenSaverParams {
            path: params.get("path").unwrap().as_str().unwrap().to_string(),
        })
    }

    async fn from_params(params: &ScreenSaverParams) -> Result<Box<Self>> {
        Ok(Box::new(ScreenSaverScreen {
            path: params.path.clone(),
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

    fn draw<D, E>(&self, target: &mut D) -> Result<(), E>
    where
        E: Debug,
        D: DrawTarget<Color = TriColor, Error = E>,
    {
        fn to_tricolor(color: &Luma<u8>) -> TriColor {
            if color[0] > 128 {
                TriColor::White
            } else {
                TriColor::Black
            }
        }

        let mut image = open(&self.path)
            .expect("Failed to open image")
            // .resize(192, 128, FilterType::Gaussian)
            .to_luma8();

        // dither(&mut image, &BiLevel);

        // resize back to full screen
        image = imageops::resize(&image, 800, 480, FilterType::Gaussian);

        let pixels = image
            .enumerate_pixels()
            .map(|(x, y, p)| Pixel(Point::new(0, 0) + Point::new(x as i32, y as i32), to_tricolor(p)));
        let _ = target.draw_iter(pixels);
        Ok(())
    }
}
