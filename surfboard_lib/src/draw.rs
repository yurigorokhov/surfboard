use core::fmt::Debug;

use crate::data::{ProgramState, TidePredictions, TIDE_PREDICTIONS_LEN};
use crate::surf_report::{self, SurfReportResponse, TideType};
use chrono::{Datelike, NaiveDateTime, TimeZone, Timelike, Utc};
use core::fmt::Write;
use embedded_graphics::mono_font::ascii::FONT_6X10;
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
            DisplayAction::DisplaySurfReport => match &state.surf_report {
                Some(surf_report) => {
                    draw_surf_report(target, &surf_report)?;
                    draw_last_updated(target, &surf_report.parse_timestamp_local().unwrap())?;
                    Ok(())
                }
                None => todo!(),
            },
        }
    }
}

pub fn draw_surf_report<D, E>(target: &mut D, surf_report: &SurfReportResponse) -> Result<(), E>
where
    E: Debug,
    D: DrawTarget<Color = TriColor, Error = E>,
{
    // find max and min
    let mut min_height: f32 = surf_report.tides.iter().map(|f| f.height).reduce(f32::min).unwrap();
    let mut max_height: f32 = surf_report.tides.iter().map(|f| f.height).reduce(f32::max).unwrap();
    let mut negative_adjustment = 0.;
    if min_height < 0. {
        negative_adjustment = -min_height;
        max_height += -min_height;
        min_height = 0.;
    }

    let mut points: Vec<Point, TIDE_PREDICTIONS_LEN> = Vec::new();

    // do not show first few hours of the night
    let skip_first_n = 1;
    let mut x_axis = TIDE_CHART_X_LEFT;
    let mut idx = 0;
    for pred in &surf_report.tides {
        if idx < skip_first_n - 1 {
            idx += 1;
            continue;
        }
        let time = Utc.timestamp_opt(pred.timestamp, 0).unwrap();
        let height = (pred.height + negative_adjustment - min_height) / max_height * TIDE_Y_HEIGHT as f32;
        let screen_height = TIDE_CHART_Y_TOP + TIDE_Y_HEIGHT - height as u32;

        points.push(Point::new(x_axis as i32, screen_height as i32)).unwrap();

        let text_style = TextStyleBuilder::new()
            .alignment(Alignment::Left)
            .line_height(LineHeight::Percent(100))
            .build();
        let mut time_label: String<8> = String::new();
        write!(time_label, "{}", time.hour()).unwrap();
        Text::with_text_style(
            time_label.as_str(),
            Point::new(x_axis as i32 - 5, TIDE_CHART_Y_BOTTOM as i32 + 50),
            MonoTextStyle::new(&FONT_5X8, TriColor::Black),
            text_style,
        )
        .draw(target)?;

        if pred.r#type == TideType::HIGH || pred.r#type == TideType::LOW {
            let mut txt: String<8> = String::new();
            write!(txt, "{:.1}ft", pred.height).unwrap();
            Text::with_text_style(
                txt.as_str(),
                Point::new(x_axis as i32 - 5, screen_height as i32 - 20),
                MonoTextStyle::new(&FONT_6X10, TriColor::Black),
                text_style,
            )
            .draw(target)?;
            Line::new(
                Point::new(x_axis as i32, TIDE_CHART_Y_BOTTOM as i32 + 30),
                Point::new(x_axis as i32, screen_height as i32),
            )
            .into_styled(PrimitiveStyle::with_stroke(TriColor::Chromatic, 2))
            .draw(target)?;
        }

        x_axis += TIDE_CHART_WIDTH / (TIDE_PREDICTIONS_LEN - skip_first_n) as u32;
        idx += 1;
    }
    Polyline::new(&points)
        .into_styled(PrimitiveStyle::with_stroke(TriColor::Black, 3))
        .draw(target)?;
    Ok(())
}

pub fn draw_last_updated<D, E>(target: &mut D, last_updated: &NaiveDateTime) -> Result<(), E>
where
    E: Debug,
    D: DrawTarget<Color = TriColor, Error = E>,
{
    let text_style = TextStyleBuilder::new()
        .alignment(Alignment::Left)
        .line_height(LineHeight::Percent(100))
        .build();
    let mut txt: String<20> = String::new();
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
