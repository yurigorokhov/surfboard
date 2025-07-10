use tokio::fs;

use embedded_graphics::prelude::Size;
use embedded_graphics_simulator::{OutputSettingsBuilder, SimulatorDisplay};
use epd_waveshare::color::TriColor;

use surfboard_scraper::{device_config::Configuration, surf_report_24h::data::SurfReport24HData};

#[tokio::test]
async fn test_surf_report() {
    let config: Configuration = serde_json::from_str(
        fs::read_to_string("deploy/config.json")
            .await
            .expect("Failed ot read config.json")
            .as_str(),
    )
    .expect("Failed to parse configuration");

    for screen in config.screens {
        let mut display = SimulatorDisplay::<TriColor>::new(Size::new(800, 480));
        let params = SurfReport24HData::parse_params(&screen.params).expect("Failed to parse parameters");
        let surf_report = SurfReport24HData::new_from_params(&params)
            .await
            .expect("Failed to fetch surf report");
        surf_report.draw(&mut display).expect("Failed to draw");

        let output_settings = OutputSettingsBuilder::new().scale(1).build();
        let output_image = display.to_grayscale_output_image(&output_settings);
        output_image
            .save_png(format!("tests/data/{}.png", screen.id))
            .expect("Failed to save test image");
    }
}
