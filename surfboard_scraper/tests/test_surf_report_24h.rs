use tokio::fs;

use embedded_graphics::prelude::Size;
use embedded_graphics_simulator::{OutputSettingsBuilder, SimulatorDisplay};
use epd_waveshare::color::TriColor;

use surfboard_scraper::surf_report_24h::surf_report::SurfReport24H;

const PLEASURE_POINT_SPOT_ID: &str = "5842041f4e65fad6a7708807";

#[tokio::test]
async fn test_surf_report() {
    let surf_report = SurfReport24H::fetch_latest(PLEASURE_POINT_SPOT_ID)
        .await
        .expect("Failed to load surf report");

    let data = serde_json::to_string_pretty(&surf_report).expect("Failed to serialize surf report");
    fs::write("tests/data/surf_report_24h.json", data)
        .await
        .expect("Unable to write file");

    // draw the report
    let mut display = SimulatorDisplay::<TriColor>::new(Size::new(800, 480));
    surf_report.draw(&mut display).expect("Failed to draw");

    let output_settings = OutputSettingsBuilder::new().scale(1).build();
    let output_image = display.to_grayscale_output_image(&output_settings);
    output_image
        .save_png("tests/data/surf_report_24h.png")
        .expect("Failed to save test image");
}
