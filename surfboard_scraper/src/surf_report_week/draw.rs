use chrono::{Datelike, NaiveDate, TimeZone, Utc};

use crate::common::draw_utils::{
    centered_text_style, draw_binary_image_on_tricolor, draw_last_updated, draw_small_text,
    draw_text, draw_weather_icon, format_wave_height, format_wind_speed,
};
use crate::image_data::{WAVE, WIND};
use core::fmt::Debug;
use embedded_graphics::image::ImageRaw;
use embedded_graphics::mono_font::ascii::FONT_9X15_BOLD;
use embedded_graphics::pixelcolor::BinaryColor;
use embedded_graphics::{mono_font::MonoTextStyle, prelude::*, primitives::{Line, PrimitiveStyle}, text::Text};
use epd_waveshare::color::TriColor;
use std::collections::HashMap;

// Weekly layout constants - 7 columns for days of the week
const CHART_X_LEFT: i32 = 40;
const CHART_X_RIGHT: i32 = 760;
const CHART_WIDTH: i32 = CHART_X_RIGHT - CHART_X_LEFT;
const COLUMN_WIDTH: i32 = CHART_WIDTH / 7;

// Better vertical centering - screen is 480px tall, footer at ~470
// Content area: ~60px from top, ~120px from bottom = 300px content area
// Balanced spacing with good visual hierarchy
const DAY_LABEL_Y: i32 = 140;        // Day names and dates
const WAVE_DATA_Y: i32 = 210;        // Wave height data  
const WEATHER_DATA_Y: i32 = 280;     // Weather icons
const WIND_DATA_Y: i32 = 350;        // Wind speed data

// Separator styling constants - span the main content area
const SEPARATOR_TOP_Y: i32 = 110;
const SEPARATOR_BOTTOM_Y: i32 = 370;

use crate::surf_report_week::data::SurfReportWeekData;

pub fn draw<D, E>(target: &mut D, surf_report: &SurfReportWeekData) -> Result<(), E>
where
    E: Debug,
    D: DrawTarget<Color = TriColor, Error = E>,
{
    // Draw day separators first (as background elements)
    draw_day_separators(target)?;

    // Draw day column headers
    draw_day_headers(target, surf_report)?;

    // Draw daily data in columns
    draw_daily_waves(target, surf_report)?;
    draw_daily_weather(target, surf_report)?;
    draw_daily_wind(target, surf_report)?;

    // Draw footer
    draw_last_updated(target, &surf_report.parse_timestamp_local().unwrap())?;

    Ok(())
}

// Remove the old tides function since we're not using it for weekly view

// Daily data grouping helpers
struct DailyWaveSummary {
    date: NaiveDate,
    min_height: i32,
    max_height: i32,
}

struct DailyWeatherSummary {
    date: NaiveDate,
    condition: crate::surfline_types::weather::WeatherCondition,
}

struct DailyWindSummary {
    date: NaiveDate,
    avg_speed: f32,
}

fn group_waves_by_day(waves: &[crate::surfline_types::wave::WaveMeasurement]) -> Vec<DailyWaveSummary> {
    let mut daily_groups: HashMap<NaiveDate, Vec<&crate::surfline_types::wave::WaveMeasurement>> = HashMap::new();

    // Group measurements by date
    for wave in waves {
        let date = Utc.timestamp_opt(wave.timestamp, 0).unwrap().date_naive();
        daily_groups.entry(date).or_insert_with(Vec::new).push(wave);
    }

    // Convert to sorted vector of daily summaries
    let mut daily_summaries: Vec<DailyWaveSummary> = daily_groups
        .into_iter()
        .map(|(date, waves)| {
            let min_height = waves.iter().map(|w| w.surf.min).min().unwrap_or(0);
            let max_height = waves.iter().map(|w| w.surf.max).max().unwrap_or(0);
            DailyWaveSummary {
                date,
                min_height,
                max_height,
            }
        })
        .collect();

    // Sort by date to ensure proper chronological order
    daily_summaries.sort_by(|a, b| a.date.cmp(&b.date));
    daily_summaries
}

fn group_weather_by_day(weather: &[crate::surfline_types::weather::WeatherMeasurement]) -> Vec<DailyWeatherSummary> {
    let mut daily_groups: HashMap<NaiveDate, Vec<&crate::surfline_types::weather::WeatherMeasurement>> = HashMap::new();

    // Group measurements by date
    for measurement in weather {
        let date = Utc.timestamp_opt(measurement.timestamp, 0).unwrap().date_naive();
        daily_groups.entry(date).or_insert_with(Vec::new).push(measurement);
    }

    // Convert to sorted vector - use midday condition as representative
    let mut daily_summaries: Vec<DailyWeatherSummary> = daily_groups
        .into_iter()
        .map(|(date, measurements)| {
            // Find measurement closest to midday (12:00)
            let midday_target = date.and_hms_opt(12, 0, 0).unwrap().and_utc().timestamp();
            let best_measurement = measurements
                .iter()
                .min_by_key(|m| (m.timestamp - midday_target).abs())
                .unwrap();
            DailyWeatherSummary {
                date,
                condition: best_measurement.condition.clone(),
            }
        })
        .collect();

    daily_summaries.sort_by(|a, b| a.date.cmp(&b.date));
    daily_summaries
}

fn group_wind_by_day(wind: &[crate::surfline_types::wind::WindMeasurement]) -> Vec<DailyWindSummary> {
    let mut daily_groups: HashMap<NaiveDate, Vec<&crate::surfline_types::wind::WindMeasurement>> = HashMap::new();

    // Group measurements by date
    for measurement in wind {
        let date = Utc.timestamp_opt(measurement.timestamp, 0).unwrap().date_naive();
        daily_groups.entry(date).or_insert_with(Vec::new).push(measurement);
    }

    // Convert to sorted vector - use average speed and most common direction
    let mut daily_summaries: Vec<DailyWindSummary> = daily_groups
        .into_iter()
        .map(|(date, measurements)| {
            let avg_speed = measurements.iter().map(|m| m.speed).sum::<f32>() / measurements.len() as f32;

            // Find most common direction type
            let mut direction_counts = HashMap::new();
            for measurement in &measurements {
                *direction_counts.entry(measurement.direction_type).or_insert(0) += 1;
            }

            DailyWindSummary { date, avg_speed }
        })
        .collect();

    daily_summaries.sort_by(|a, b| a.date.cmp(&b.date));
    daily_summaries
}

pub fn draw_day_headers<D, E>(target: &mut D, surf_report: &SurfReportWeekData) -> Result<(), E>
where
    E: Debug,
    D: DrawTarget<Color = TriColor, Error = E>,
{
    let text_style = centered_text_style();

    // Get dates from wave data
    let daily_waves = group_waves_by_day(&surf_report.waves);

    for (day_index, wave_summary) in daily_waves.iter().enumerate().take(7) {
        let x_pos = CHART_X_LEFT + (day_index as i32 * COLUMN_WIDTH) + (COLUMN_WIDTH / 2);

        // Get the weekday name based on the actual date
        let weekday_name = match wave_summary.date.weekday() {
            chrono::Weekday::Mon => "Mon",
            chrono::Weekday::Tue => "Tue",
            chrono::Weekday::Wed => "Wed",
            chrono::Weekday::Thu => "Thu",
            chrono::Weekday::Fri => "Fri",
            chrono::Weekday::Sat => "Sat",
            chrono::Weekday::Sun => "Sun",
        };

        // Format date as M/D (e.g., "8/8" for August 8th)
        let date_str = format!("{}/{}", wave_summary.date.month(), wave_summary.date.day());

        // Draw day name
        Text::with_text_style(
            weekday_name,
            Point::new(x_pos, DAY_LABEL_Y),
            MonoTextStyle::new(&FONT_9X15_BOLD, TriColor::Black),
            text_style,
        )
        .draw(target)?;

        // Draw date below day name
        draw_small_text(target, &date_str, Point::new(x_pos, DAY_LABEL_Y + 20), text_style)?;
    }

    Ok(())
}

pub fn draw_daily_waves<D, E>(target: &mut D, surf_report: &SurfReportWeekData) -> Result<(), E>
where
    E: Debug,
    D: DrawTarget<Color = TriColor, Error = E>,
{
    // Draw wave icon
    draw_binary_image_on_tricolor(
        &ImageRaw::<BinaryColor>::new(WAVE, 32),
        Point::new(10, WAVE_DATA_Y - 16),
        target,
    );

    let daily_waves = group_waves_by_day(&surf_report.waves);
    let text_style = centered_text_style();

    for (day_index, wave_summary) in daily_waves.iter().enumerate().take(7) {
        let x_pos = CHART_X_LEFT + (day_index as i32 * COLUMN_WIDTH) + (COLUMN_WIDTH / 2);

        // Draw wave height text (e.g., "2-4ft")
        let height_text = format_wave_height(wave_summary.min_height, wave_summary.max_height);
        draw_text(target, &height_text, Point::new(x_pos, WAVE_DATA_Y), text_style)?;
    }

    Ok(())
}

pub fn draw_daily_weather<D, E>(target: &mut D, surf_report: &SurfReportWeekData) -> Result<(), E>
where
    E: Debug,
    D: DrawTarget<Color = TriColor, Error = E>,
{
    let daily_weather = group_weather_by_day(&surf_report.weather);

    for (day_index, weather_summary) in daily_weather.iter().enumerate().take(7) {
        let x_pos = CHART_X_LEFT + (day_index as i32 * COLUMN_WIDTH) + (COLUMN_WIDTH / 2);
        draw_weather_icon(&weather_summary.condition, Point::new(x_pos, WEATHER_DATA_Y), target);
    }

    Ok(())
}

pub fn draw_daily_wind<D, E>(target: &mut D, surf_report: &SurfReportWeekData) -> Result<(), E>
where
    E: Debug,
    D: DrawTarget<Color = TriColor, Error = E>,
{
    // Draw wind icon
    draw_binary_image_on_tricolor(
        &ImageRaw::<BinaryColor>::new(WIND, 32),
        Point::new(10, WIND_DATA_Y - 16),
        target,
    );

    let daily_wind = group_wind_by_day(&surf_report.wind);
    let text_style = centered_text_style();

    for (day_index, wind_summary) in daily_wind.iter().enumerate().take(7) {
        let x_pos = CHART_X_LEFT + (day_index as i32 * COLUMN_WIDTH) + (COLUMN_WIDTH / 2);

        // Draw wind speed text
        let speed_text = format_wind_speed(wind_summary.avg_speed);
        draw_text(target, &speed_text, Point::new(x_pos, WIND_DATA_Y), text_style)?;
    }

    Ok(())
}

pub fn draw_weather<D, E>(
    _target: &mut D,
    _surf_report: &SurfReportWeekData,
    _min_time: i64,
    _max_time: i64,
    _y: i32,
) -> Result<(), E>
where
    E: Debug,
    D: DrawTarget<Color = TriColor, Error = E>,
{
    // This function is kept for compatibility but is replaced by draw_daily_weather
    Ok(())
}

pub fn draw_day_separators<D, E>(target: &mut D) -> Result<(), E>
where
    E: Debug,
    D: DrawTarget<Color = TriColor, Error = E>,
{
    // Create a thin line style for subtle separators
    let thin_line_style = PrimitiveStyle::with_stroke(TriColor::Black, 1);
    
    // Draw vertical separators between day columns (but not after the last column)
    for day_index in 1..7 {
        let x_pos = CHART_X_LEFT + (day_index as i32 * COLUMN_WIDTH);
        
        // Draw main vertical separator line
        Line::new(
            Point::new(x_pos, SEPARATOR_TOP_Y),
            Point::new(x_pos, SEPARATOR_BOTTOM_Y),
        )
        .into_styled(thin_line_style)
        .draw(target)?;
        
        // Add small decorative elements at key intersections
        // Top accent dot
        target.draw_iter([Pixel(Point::new(x_pos, SEPARATOR_TOP_Y - 5), TriColor::Black)])?;
        
        // Middle accent dots at each data row
        target.draw_iter([Pixel(Point::new(x_pos, WAVE_DATA_Y), TriColor::Black)])?;
        target.draw_iter([Pixel(Point::new(x_pos, WEATHER_DATA_Y), TriColor::Black)])?;
        target.draw_iter([Pixel(Point::new(x_pos, WIND_DATA_Y), TriColor::Black)])?;
        
        // Bottom accent dot
        target.draw_iter([Pixel(Point::new(x_pos, SEPARATOR_BOTTOM_Y + 5), TriColor::Black)])?;
    }
    
    Ok(())
}


