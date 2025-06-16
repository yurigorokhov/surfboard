use chrono::NaiveDate;
use embedded_graphics::prelude::Size;
use embedded_graphics_simulator::{OutputSettingsBuilder, SimulatorDisplay};
use epd_waveshare::color::TriColor;
use std::fs;
use surfboard_lib::{
    data::TidePredictions,
    draw::{draw_last_updated, draw_tide},
};

#[tokio::test]
async fn test_drawing_tide() {
    let mut display = SimulatorDisplay::<TriColor>::new(Size::new(800, 480));

    let contents =
        fs::read_to_string("tests/data/tide_predictions.json").expect("Should have been able to read the file");
    let (data, _remainder) = serde_json_core::from_str::<TidePredictions>(contents.as_str()).unwrap();

    draw_tide(&mut display, &data).expect("Failed to draw loading screen");
    let output_settings = OutputSettingsBuilder::new().scale(1).build();
    let output_image = display.to_grayscale_output_image(&output_settings);
    output_image
        .save_png("tests/screenshots/tide_predictions.png")
        .expect("Failed to save test image");
}

#[tokio::test]
async fn test_draw_last_updated() {
    let dt = NaiveDate::from_ymd_opt(2025, 01, 01)
        .unwrap()
        .and_hms_milli_opt(9, 10, 11, 12)
        .unwrap();

    let mut display = SimulatorDisplay::<TriColor>::new(Size::new(800, 480));
    draw_last_updated(&mut display, &dt).expect("Failed to draw loading screen");
    let output_settings = OutputSettingsBuilder::new().scale(1).build();
    let output_image = display.to_grayscale_output_image(&output_settings);
    output_image
        .save_png("tests/screenshots/last_updated.png")
        .expect("Failed to save test image");
}
