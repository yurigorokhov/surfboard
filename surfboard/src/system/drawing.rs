use core::fmt::Debug;

use embedded_graphics::mono_font::iso_8859_10::FONT_10X20;
use embedded_graphics::pixelcolor::Rgb888;
use embedded_graphics::{
    mono_font::MonoTextStyle,
    prelude::*,
    text::{Alignment, LineHeight, Text, TextStyleBuilder},
};
use epd_waveshare::color::Color;
use heapless::String;
use tinyqoi::Qoi;

use crate::task::state::ProgramState;

#[derive(PartialEq)]
pub enum DisplayCommand {
    ShowStatusText(String<30>, i32),
    DrawImage(usize),
    DisplayPowerOff,
}

impl DisplayCommand {
    pub fn draw<D, E>(&self, target: &mut D, state: &ProgramState) -> Result<(), E>
    where
        E: Debug,
        D: DrawTarget<Color = Color, Error = E>,
    {
        match self {
            DisplayCommand::ShowStatusText(text, line_number) => {
                let text_style = TextStyleBuilder::new()
                    .alignment(Alignment::Left)
                    .line_height(LineHeight::Percent(150))
                    .build();
                Text::with_text_style(
                    text.as_str(),
                    Point::new(20, 20 + (16 * line_number)),
                    MonoTextStyle::new(&FONT_10X20, Color::White),
                    text_style,
                )
                .draw(target)?;
                Ok(())
            }
            DisplayCommand::DrawImage(screen_idx) => {
                let zero = Point::zero();
                let image = Qoi::new(state.get_buffer_for_screen(*screen_idx).unwrap()).expect("Failed to parse image");
                let pixels = image
                    .pixels()
                    .enumerate()
                    .map(|(i, p)| Pixel(zero + Point::new(i as i32 % 800, i as i32 / 800), rgb888_to_bw(p)));
                let _ = target.draw_iter(pixels);
                Ok(())
            }
            DisplayCommand::DisplayPowerOff => Ok(()),
        }
    }
}

fn rgb888_to_bw(color: Rgb888) -> Color {
    // Use luminance formula: 0.299*R + 0.587*G + 0.114*B
    let luminance = 0.299 * (color.r() as f32) + 0.587 * (color.g() as f32) + 0.114 * (color.b() as f32);
    if luminance > 128.0 {
        Color::Black
    } else {
        Color::White
    }
}
