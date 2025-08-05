use chrono::Timelike;

use crate::common::draw_utils::{
    draw_binary_image_on_tricolor, draw_last_updated, draw_weather_icon, format_wave_height,
    format_wind_speed, get_local_time_from_unix, left_text_style,
};
use crate::image_data::{WAVE, WIND};
use core::fmt::Debug;
use core::fmt::Write;
use embedded_graphics::image::ImageRaw;
use embedded_graphics::mono_font::ascii::{FONT_7X13, FONT_8X13, FONT_9X15, FONT_9X15_BOLD};
use embedded_graphics::mono_font::iso_8859_10::FONT_10X20;
use embedded_graphics::pixelcolor::BinaryColor;
use embedded_graphics::primitives::{Line, Polyline, PrimitiveStyle};
use embedded_graphics::{
    mono_font::MonoTextStyle,
    prelude::*,
    text::{Alignment, LineHeight, Text, TextStyleBuilder},
};
use epd_waveshare::color::TriColor;
const TIDE_CHART_X_LEFT: i32 = 50;
const TIDE_CHART_X_RIGHT: i32 = 760;
const TIDE_CHART_WIDTH: i32 = TIDE_CHART_X_RIGHT - TIDE_CHART_X_LEFT;
const TIDE_CHART_Y_TOP: i32 = 100;
const TIDE_CHART_Y_BOTTOM: i32 = 220;
const TIDE_Y_HEIGHT: i32 = TIDE_CHART_Y_BOTTOM - TIDE_CHART_Y_TOP;

use crate::surf_report_24h::data::SurfReport24HData;
use crate::surfline_types::wind::WindDirectionType;


pub fn draw<D, E>(target: &mut D, surf_report: &SurfReport24HData) -> Result<(), E>
where
    E: Debug,
    D: DrawTarget<Color = TriColor, Error = E>,
{
    let (min_time, max_time) = draw_tides(target, &surf_report)?;
    draw_weather(target, surf_report, min_time, max_time, 350)?;
    draw_wind(target, surf_report, min_time, max_time, 400)?;
    draw_wave_height(target, &surf_report, min_time, max_time, 450)?;
    draw_headings(target, &surf_report, 20)?;
    draw_last_updated(target, &surf_report.parse_timestamp_local().unwrap())?;
    Ok(())
}

pub fn draw_tides<D, E>(target: &mut D, surf_report: &SurfReport24HData) -> Result<(i64, i64), E>
where
    E: Debug,
    D: DrawTarget<Color = TriColor, Error = E>,
{
    // find max and min
    let mut min_height: f32 = surf_report.tides.iter().map(|f| f.height).reduce(f32::min).unwrap();
    let mut max_height: f32 = surf_report.tides.iter().map(|f| f.height).reduce(f32::max).unwrap();
    let min_time: i64 = surf_report.tides.iter().map(|f| f.timestamp).reduce(i64::min).unwrap();
    let max_time: i64 = surf_report.tides.iter().map(|f| f.timestamp).reduce(i64::max).unwrap();
    let mut negative_adjustment = 0.;
    if min_height < 0. {
        negative_adjustment = -min_height;
        max_height += -min_height;
        min_height = 0.;
    }

    let mut points: Vec<Point> = Vec::new();

    let mut idx: usize = 0;
    let mut skip_next_ts = false;
    for pred in &surf_report.tides {
        // if next data point is low/high tide, skip drawing this one
        if idx < surf_report.tides.len() - 1 && surf_report.tides[idx + 1].r#type.is_high_low() {
            idx += 1;
            continue;
        }

        // if previous data point is high/low, skip drawing this one
        if idx > 0 && surf_report.tides[idx - 1].r#type.is_high_low() {
            idx += 1;
            continue;
        }
        idx += 1;

        // let pixels_per_foot = (pred.height + negative_adjustment - min_height) / max_height * TIDE_Y_HEIGHT as f32;
        let height = (pred.height + negative_adjustment - min_height) / max_height * TIDE_Y_HEIGHT as f32;
        let screen_height = TIDE_CHART_Y_TOP + TIDE_Y_HEIGHT - height as i32;

        let x_axis_proportion = (pred.timestamp as f64 - min_time as f64) / (max_time - min_time) as f64;
        let x_axis = (TIDE_CHART_X_LEFT as f64 + (TIDE_CHART_WIDTH as f64) * x_axis_proportion) as i32;
        points.push(Point::new(x_axis, screen_height));

        let text_style = TextStyleBuilder::new()
            .alignment(Alignment::Left)
            .line_height(LineHeight::Percent(100))
            .build();
        let local_time = get_local_time_from_unix(pred.timestamp, pred.utc_offset);

        // show timestamp only if it is a low/high tide, or a weather event
        if !skip_next_ts && (pred.r#type.is_high_low() || local_time.hour() % 3 == 0) {
            let mut time_label: String = String::new();
            if pred.r#type.is_high_low() {
                // show minutes for high/low tide
                write!(time_label, "{:.2}:{:02}", local_time.hour(), local_time.minute()).unwrap();
                skip_next_ts = true;
            } else {
                write!(time_label, "{:.2}", local_time.hour()).unwrap();
            }
            Text::with_text_style(
                time_label.as_str(),
                Point::new(x_axis - 5, TIDE_CHART_Y_BOTTOM + 50),
                MonoTextStyle::new(
                    if pred.r#type.is_high_low() {
                        &FONT_9X15_BOLD
                    } else {
                        &FONT_8X13
                    },
                    if pred.r#type.is_high_low() {
                        TriColor::Chromatic
                    } else {
                        TriColor::Black
                    },
                ),
                text_style,
            )
            .draw(target)?;
        } else {
            skip_next_ts = false;
        }

        if pred.r#type.is_high_low() {
            let mut txt: String = String::new();
            write!(txt, "{:.1}ft", pred.height).unwrap();
            Text::with_text_style(
                txt.as_str(),
                Point::new(x_axis - 5, screen_height - 20),
                MonoTextStyle::new(&FONT_7X13, TriColor::Black),
                text_style,
            )
            .draw(target)?;
            Line::new(
                Point::new(x_axis, TIDE_CHART_Y_BOTTOM + 30),
                Point::new(x_axis, screen_height),
            )
            .into_styled(PrimitiveStyle::with_stroke(TriColor::Chromatic, 2))
            .draw(target)?;
        }
    }
    Polyline::new(&points)
        .into_styled(PrimitiveStyle::with_stroke(TriColor::Black, 3))
        .draw(target)?;
    Ok((min_time, max_time))
}

pub fn draw_wave_height<D, E>(
    target: &mut D,
    surf_report: &SurfReport24HData,
    min_time: i64,
    max_time: i64,
    y: i32,
) -> Result<(), E>
where
    E: Debug,
    D: DrawTarget<Color = TriColor, Error = E>,
{
    draw_binary_image_on_tricolor(&ImageRaw::<BinaryColor>::new(WAVE, 32), Point::new(10, y - 16), target);

    let text_style = left_text_style();
    for data in surf_report.waves.iter().take(10) {
        let x_axis_proportion = (data.timestamp as f64 - min_time as f64) / (max_time - min_time) as f64;
        let x_axis = (TIDE_CHART_X_LEFT as f64 + (TIDE_CHART_WIDTH as f64) * x_axis_proportion) as i32;

        let wave_text = format_wave_height(data.surf.min, data.surf.max);
        Text::with_text_style(
            wave_text.as_str(),
            Point::new(x_axis - 10, y),
            MonoTextStyle::new(&FONT_9X15, TriColor::Black),
            text_style,
        )
        .draw(target)?;
    }
    Ok(())
}

pub fn draw_wind<D, E>(
    target: &mut D,
    surf_report: &SurfReport24HData,
    min_time: i64,
    max_time: i64,
    y: i32,
) -> Result<(), E>
where
    E: Debug,
    D: DrawTarget<Color = TriColor, Error = E>,
{
    draw_binary_image_on_tricolor(&ImageRaw::<BinaryColor>::new(WIND, 32), Point::new(10, y - 16), target);

    let text_style = left_text_style();

    for data in surf_report.wind.iter().take(10) {
        let x_axis_proportion = (data.timestamp as f64 - min_time as f64) / (max_time - min_time) as f64;
        let x_axis = (TIDE_CHART_X_LEFT as f64 + (TIDE_CHART_WIDTH as f64) * x_axis_proportion) as i32;

        let wind_text = format_wind_speed(data.speed);
        Text::with_text_style(
            wind_text.as_str(),
            Point::new(x_axis - 10, y),
            MonoTextStyle::new(&FONT_9X15, TriColor::Black),
            text_style,
        )
        .draw(target)?;
        const ARROW_OFFSET_X: i32 = 45;
        if data.direction_type == WindDirectionType::CrossShore {
            let l = Line::with_delta(Point::new(x_axis + ARROW_OFFSET_X, y - 2), Point::new(10, 0));
            l.into_styled(PrimitiveStyle::with_stroke(TriColor::Black, 1))
                .draw(target)?;
            Line::with_delta(l.end, Point::new(-3, -3))
                .into_styled(PrimitiveStyle::with_stroke(TriColor::Black, 1))
                .draw(target)?;
            Line::with_delta(l.end, Point::new(-3, 3))
                .into_styled(PrimitiveStyle::with_stroke(TriColor::Black, 1))
                .draw(target)?;
            Line::with_delta(l.start, Point::new(3, -3))
                .into_styled(PrimitiveStyle::with_stroke(TriColor::Black, 1))
                .draw(target)?;
            Line::with_delta(l.start, Point::new(3, 3))
                .into_styled(PrimitiveStyle::with_stroke(TriColor::Black, 1))
                .draw(target)?;
        } else if data.direction_type == WindDirectionType::Onshore {
            let l = Line::with_delta(Point::new(x_axis + ARROW_OFFSET_X, y - 8), Point::new(0, 10));
            l.into_styled(PrimitiveStyle::with_stroke(TriColor::Black, 1))
                .draw(target)?;
            Line::with_delta(l.end, Point::new(3, -3))
                .into_styled(PrimitiveStyle::with_stroke(TriColor::Black, 1))
                .draw(target)?;
            Line::with_delta(l.end, Point::new(-3, -3))
                .into_styled(PrimitiveStyle::with_stroke(TriColor::Black, 1))
                .draw(target)?;
        } else if data.direction_type == WindDirectionType::Offshore {
            let l = Line::with_delta(Point::new(x_axis + ARROW_OFFSET_X, y - 8), Point::new(0, 10));
            l.into_styled(PrimitiveStyle::with_stroke(TriColor::Black, 1))
                .draw(target)?;
            Line::with_delta(l.start, Point::new(3, 3))
                .into_styled(PrimitiveStyle::with_stroke(TriColor::Black, 1))
                .draw(target)?;
            Line::with_delta(l.start, Point::new(-3, 3))
                .into_styled(PrimitiveStyle::with_stroke(TriColor::Black, 1))
                .draw(target)?;
        }
    }
    Ok(())
}

pub fn draw_weather<D, E>(
    target: &mut D,
    surf_report: &SurfReport24HData,
    min_time: i64,
    max_time: i64,
    y: i32,
) -> Result<(), E>
where
    E: Debug,
    D: DrawTarget<Color = TriColor, Error = E>,
{
    for data in surf_report.weather.iter().take(10) {
        let x_axis_proportion = (data.timestamp as f64 - min_time as f64) / (max_time - min_time) as f64;
        let x_axis = (TIDE_CHART_X_LEFT as f64 + (TIDE_CHART_WIDTH as f64) * x_axis_proportion) as i32;

        draw_weather_icon(&data.condition, Point::new(x_axis, y), target);
    }
    Ok(())
}


pub fn draw_headings<D, E>(target: &mut D, surf_report: &SurfReport24HData, y: i32) -> Result<(), E>
where
    E: Debug,
    D: DrawTarget<Color = TriColor, Error = E>,
{
    let text_style = left_text_style();
    Text::with_text_style(
        surf_report.spot_details.name.as_str(),
        Point::new(10, y),
        MonoTextStyle::new(&FONT_10X20, TriColor::Black),
        text_style,
    )
    .draw(target)?;
    Text::with_text_style(
        surf_report.conditions.headline.as_str(),
        Point::new(10, y + 20),
        MonoTextStyle::new(&FONT_8X13, TriColor::Black),
        text_style,
    )
    .draw(target)?;
    Ok(())
}

