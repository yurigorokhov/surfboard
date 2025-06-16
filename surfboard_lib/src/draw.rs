use core::fmt::Debug;

use crate::data::{ProgramState, TidePredictions, TIDE_PREDICTIONS_LEN};
use core::fmt::Write;
use embedded_graphics::mono_font::iso_8859_10::FONT_10X20;
use embedded_graphics::mono_font::iso_8859_16::FONT_5X8;
use embedded_graphics::primitives::{Line, Polyline, PrimitiveStyle};
use embedded_graphics::{
    mono_font::MonoTextStyle,
    prelude::*,
    text::{Alignment, LineHeight, Text, TextStyleBuilder},
};
use epd_waveshare::color::TriColor;
use heapless::{String, Vec};
const TIDE_CHART_X_LEFT: u32 = 20;
const TIDE_CHART_X_RIGHT: u32 = 780;
const TIDE_CHART_WIDTH: u32 = TIDE_CHART_X_RIGHT - TIDE_CHART_X_LEFT;
const TIDE_CHART_Y_TOP: u32 = 200;
const TIDE_CHART_Y_BOTTOM: u32 = 400;
const TIDE_Y_HEIGHT: u32 = TIDE_CHART_Y_BOTTOM - TIDE_CHART_Y_TOP;

pub enum DisplayAction {
    ShowStatusText(String<30>),
    DisplaySurfReport,
    Clear,
}

impl DisplayAction {
    pub fn draw<D, E>(self, target: &mut D, state: &ProgramState) -> Result<(), E>
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
            DisplayAction::DisplaySurfReport => {
                match &state.tide_predictions {
                    Some(predictions) => {
                        draw_tide(target, &predictions)?;
                    }
                    None => todo!(),
                }
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

pub fn draw_tide<D, E>(target: &mut D, tide_predictions: &TidePredictions) -> Result<(), E>
where
    E: Debug,
    D: DrawTarget<Color = TriColor, Error = E>,
{
    // find max and min
    let heights: Vec<f32, TIDE_PREDICTIONS_LEN> = tide_predictions
        .predictions
        .iter()
        .map(|p| lexical::parse(&p.v).unwrap())
        .collect();

    let mut min_height: f32 = heights.iter().map(|f| *f).reduce(f32::min).unwrap();
    let mut max_height: f32 = heights.iter().map(|f| *f).reduce(f32::max).unwrap();
    let mut negative_adjustment = 0.;
    if min_height < 0. {
        negative_adjustment = -min_height;
        max_height += -min_height;
        min_height = 0.;
    }

    let mut points: Vec<Point, TIDE_PREDICTIONS_LEN> = Vec::new();

    // do not show first few hours of the night
    let skip_first_n = 6;
    let mut x_axis = TIDE_CHART_X_LEFT;
    let mut idx = 0;
    for pred in &tide_predictions.predictions {
        if idx < skip_first_n - 1 {
            idx += 1;
            continue;
        }
        let height = (heights[idx] + negative_adjustment - min_height) / max_height * TIDE_Y_HEIGHT as f32;
        let screen_height = TIDE_CHART_Y_TOP + TIDE_Y_HEIGHT - height as u32;

        points.push(Point::new(x_axis as i32, screen_height as i32)).unwrap();

        let text_style = TextStyleBuilder::new()
            .alignment(Alignment::Left)
            .line_height(LineHeight::Percent(100))
            .build();
        Text::with_text_style(
            &pred.t[11..13],
            Point::new(x_axis as i32 - 5, TIDE_CHART_Y_BOTTOM as i32 + 50),
            MonoTextStyle::new(&FONT_5X8, TriColor::Black),
            text_style,
        )
        .draw(target)?;
        let mut txt: String<8> = String::new();
        write!(txt, "{:.1}ft", heights[idx]).unwrap();
        Text::with_text_style(
            txt.as_str(),
            Point::new(x_axis as i32 - 5, screen_height as i32 - 20),
            MonoTextStyle::new(&FONT_5X8, TriColor::Black),
            text_style,
        )
        .draw(target)?;

        Line::new(
            Point::new(x_axis as i32, TIDE_CHART_Y_BOTTOM as i32 + 30),
            Point::new(x_axis as i32, screen_height as i32),
        )
        .into_styled(PrimitiveStyle::with_stroke(TriColor::Chromatic, 2))
        .draw(target)?;

        x_axis += TIDE_CHART_WIDTH / (TIDE_PREDICTIONS_LEN - skip_first_n) as u32;
        idx += 1;
    }
    Polyline::new(&points)
        .into_styled(PrimitiveStyle::with_stroke(TriColor::Black, 3))
        .draw(target)?;
    Ok(())
}
