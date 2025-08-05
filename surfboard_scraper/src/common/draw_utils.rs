use chrono::{Datelike, FixedOffset, NaiveDateTime, TimeZone, Timelike, Utc};
use core::fmt::Debug;
use core::fmt::Write;
use embedded_graphics::image::GetPixel;
use embedded_graphics::image::ImageRaw;
use embedded_graphics::mono_font::ascii::{FONT_8X13, FONT_9X15};
use embedded_graphics::mono_font::iso_8859_16::FONT_5X8;
use embedded_graphics::pixelcolor::BinaryColor;
use embedded_graphics::{
    mono_font::MonoTextStyle,
    prelude::*,
    text::{Alignment, LineHeight, Text, TextStyleBuilder},
};
use epd_waveshare::color::TriColor;

use crate::image_data::{MIST, MOSTLY_CLEAR, MOSTLY_CLOUDY, SHOWERS, WEATHER_SUNNY};
use crate::surfline_types::weather::WeatherCondition;

/// Common function to draw binary images on tri-color displays
pub fn draw_binary_image_on_tricolor<D>(raw_image: &ImageRaw<BinaryColor>, top_left: Point, target: &mut D)
where
    D: DrawTarget<Color = TriColor>,
{
    for p in raw_image.bounding_box().points() {
        let color = raw_image.pixel(p).unwrap();
        let mapped_color = match color {
            BinaryColor::Off => TriColor::White,
            BinaryColor::On => TriColor::Black,
        };
        let _ = target.draw_iter([Pixel(top_left + p, mapped_color)]);
    }
}

/// Convert Unix timestamp to local time with timezone offset
pub fn get_local_time_from_unix(unix_timestamp: i64, offset: i32) -> NaiveDateTime {
    let time = Utc.timestamp_opt(unix_timestamp, 0).unwrap().naive_local();
    let offset = FixedOffset::west_opt(offset * 3600).unwrap();
    offset.from_local_datetime(&time).unwrap().naive_utc()
}

/// Common function to draw the last updated timestamp
pub fn draw_last_updated<D, E>(target: &mut D, last_updated: &NaiveDateTime) -> Result<(), E>
where
    E: Debug,
    D: DrawTarget<Color = TriColor, Error = E>,
{
    let text_style = TextStyleBuilder::new()
        .alignment(Alignment::Left)
        .line_height(LineHeight::Percent(100))
        .build();
    let mut txt: String = String::new();
    write!(
        txt,
        "Updated: {}/{} {:02}:{:02}",
        last_updated.month(),
        last_updated.day(),
        last_updated.hour(),
        last_updated.minute()
    )
    .expect("Failed to write");
    Text::with_text_style(
        txt.as_str(),
        Point::new(680, 470),
        MonoTextStyle::new(&FONT_5X8, TriColor::Black),
        text_style,
    )
    .draw(target)?;
    Ok(())
}

/// Common function to draw weather condition icons
pub fn draw_weather_icon<D>(condition: &WeatherCondition, position: Point, target: &mut D)
where
    D: DrawTarget<Color = TriColor>,
{
    match condition {
        WeatherCondition::NightClear | WeatherCondition::Clear => {
            draw_binary_image_on_tricolor(
                &ImageRaw::<BinaryColor>::new(WEATHER_SUNNY, 32),
                Point::new(position.x - 16, position.y - 16),
                target,
            );
        }
        WeatherCondition::MostlyClear | WeatherCondition::NightMostlyClear => {
            draw_binary_image_on_tricolor(
                &ImageRaw::<BinaryColor>::new(MOSTLY_CLEAR, 32),
                Point::new(position.x - 16, position.y - 16),
                target,
            );
        }
        WeatherCondition::NightMostlyCloudy | WeatherCondition::MostlyCloudy | WeatherCondition::NightCloudy => {
            draw_binary_image_on_tricolor(
                &ImageRaw::<BinaryColor>::new(MOSTLY_CLOUDY, 32),
                Point::new(position.x - 16, position.y - 16),
                target,
            );
        }
        WeatherCondition::Mist | WeatherCondition::NightMist | WeatherCondition::NightFog => {
            draw_binary_image_on_tricolor(
                &ImageRaw::<BinaryColor>::new(MIST, 32),
                Point::new(position.x - 16, position.y - 16),
                target,
            );
        }
        WeatherCondition::NightBriefShowers
        | WeatherCondition::BriefShowers
        | WeatherCondition::BriefShowersPossible
        | WeatherCondition::NightDrizzle
        | WeatherCondition::Drizzle => {
            draw_binary_image_on_tricolor(
                &ImageRaw::<BinaryColor>::new(SHOWERS, 32),
                Point::new(position.x - 16, position.y - 16),
                target,
            );
        }
    }
}

/// Create a common text style for centered text
pub fn centered_text_style() -> embedded_graphics::text::TextStyle {
    TextStyleBuilder::new()
        .alignment(Alignment::Center)
        .line_height(LineHeight::Percent(100))
        .build()
}

/// Create a common text style for left-aligned text
pub fn left_text_style() -> embedded_graphics::text::TextStyle {
    TextStyleBuilder::new()
        .alignment(Alignment::Left)
        .line_height(LineHeight::Percent(100))
        .build()
}

/// Format wave height range as a string (e.g., "2-4ft")
pub fn format_wave_height(min_height: i32, max_height: i32) -> String {
    let mut height_text = String::new();
    write!(height_text, "{}-{}ft", min_height, max_height).unwrap();
    height_text
}

/// Format wind speed as a string (e.g., "12kt")
pub fn format_wind_speed(speed: f32) -> String {
    let mut speed_text = String::new();
    write!(speed_text, "{:.0}kt", speed).unwrap();
    speed_text
}

/// Draw text with standard font and color
pub fn draw_text<D, E>(
    target: &mut D,
    text: &str,
    position: Point,
    style: embedded_graphics::text::TextStyle,
) -> Result<(), E>
where
    E: Debug,
    D: DrawTarget<Color = TriColor, Error = E>,
{
    Text::with_text_style(text, position, MonoTextStyle::new(&FONT_9X15, TriColor::Black), style).draw(target)?;
    Ok(())
}

/// Draw text with small font
pub fn draw_small_text<D, E>(
    target: &mut D,
    text: &str,
    position: Point,
    style: embedded_graphics::text::TextStyle,
) -> Result<(), E>
where
    E: Debug,
    D: DrawTarget<Color = TriColor, Error = E>,
{
    Text::with_text_style(text, position, MonoTextStyle::new(&FONT_8X13, TriColor::Black), style).draw(target)?;
    Ok(())
}
