use crate::message::data::MessageData;
use core::fmt::Debug;
use embedded_graphics::mono_font::ascii::FONT_10X20;
use embedded_graphics::mono_font::MonoTextStyle;
use embedded_graphics::prelude::*;
use embedded_graphics::text::{Alignment, Text};
use epd_waveshare::color::TriColor;

pub fn draw<D, E>(target: &mut D, data: &MessageData) -> Result<(), E>
where
    E: Debug,
    D: DrawTarget<Color = TriColor, Error = E>,
{
    let style = MonoTextStyle::new(&FONT_10X20, TriColor::Black);

    // Draw "Happy Birthday" centered
    Text::with_alignment(
        data.message.as_str(),
        Point::new(400, 200),
        style,
        Alignment::Center,
    )
    .draw(target)?;

    Ok(())
}
