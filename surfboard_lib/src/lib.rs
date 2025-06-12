#![no_std]

use core::fmt::Debug;

use embedded_graphics::mono_font::iso_8859_10::FONT_10X20;
use embedded_graphics::{
    mono_font::MonoTextStyle,
    prelude::*,
    text::{Alignment, LineHeight, Text, TextStyleBuilder},
};
use epd_waveshare::color::TriColor;
use heapless::String;

pub enum DisplayAction {
    ShowStatusText(String<20>),
    Clear,
}

impl DisplayAction {
    pub fn draw<D, E>(self, target: &mut D) -> Result<(), E>
    where
        E: Debug,
        D: DrawTarget<Color = TriColor, Error = E>,
    {
        match self {
            DisplayAction::ShowStatusText(text) => {
                let text_style = TextStyleBuilder::new()
                    .alignment(Alignment::Left)
                    .line_height(LineHeight::Percent(150))
                    .build();
                Text::with_text_style(
                    text.as_str(),
                    Point::new(20, 30),
                    MonoTextStyle::new(&FONT_10X20, TriColor::Black),
                    text_style,
                )
                .draw(target)?;
                Ok(())
            }
            DisplayAction::Clear => {
                target.clear(epd_waveshare::color::TriColor::White).unwrap();
                Ok(())
            }
        }
    }
}

pub fn draw_loading_screen<D, E>(target: &mut D) -> Result<(), E>
where
    E: Debug,
    D: DrawTarget<Color = TriColor, Error = E>,
{
    // Create a new text style.
    let text_style = TextStyleBuilder::new()
        .alignment(Alignment::Left)
        .line_height(LineHeight::Percent(150))
        .build();

    // Create a text at position (20, 30) and draw it using the previously defined style.
    Text::with_text_style(
        "Hello from embedded Rust!",
        Point::new(20, 30),
        MonoTextStyle::new(&FONT_10X20, TriColor::Black),
        text_style,
    )
    .draw(target)?;
    Text::with_text_style(
        "- Yuri Gorokhov",
        Point::new(20, 50),
        MonoTextStyle::new(&FONT_10X20, TriColor::Chromatic),
        text_style,
    )
    .draw(target)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use embedded_graphics_simulator::{OutputSettingsBuilder, SimulatorDisplay};

    use super::*;

    #[test]
    fn test_loading_screen() {
        let mut display = SimulatorDisplay::<TriColor>::new(Size::new(800, 480));
        draw_loading_screen(&mut display).expect("Failed to draw loading screen");
        let output_settings = OutputSettingsBuilder::new().scale(1).build();
        let output_image = display.to_grayscale_output_image(&output_settings);
        output_image
            .save_png("tests_screenshots/loading_screen.png")
            .expect("Failed to save test image");
    }
}
