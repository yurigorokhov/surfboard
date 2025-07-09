use tokio::fs;

use embedded_graphics::prelude::Size;
use embedded_graphics_simulator::{OutputSettingsBuilder, SimulatorDisplay};
use epd_waveshare::color::TriColor;
use surfboard_scraper::surf_report_24h::draw::draw;

use surfboard_scraper::{
    surf_report_24h::conditions::fetch_conditions, surf_report_24h::spot_details::fetch_spot_details,
    surf_report_24h::surf_report::SurfReport, surf_report_24h::tide::fetch_tides, surf_report_24h::wave::fetch_waves,
    surf_report_24h::weather::fetch_weather, surf_report_24h::wind::fetch_wind,
};

const PLEASURE_POINT_SPOT_ID: &str = "5842041f4e65fad6a7708807";

#[tokio::test]
async fn test_surf_report() {
    let tides_result = fetch_tides(PLEASURE_POINT_SPOT_ID).await.expect("Failed to load tides");
    let wave_result = fetch_waves(PLEASURE_POINT_SPOT_ID).await.expect("Failed to load waves");
    let weather_result = fetch_weather(PLEASURE_POINT_SPOT_ID)
        .await
        .expect("Failed to load weather");
    let wind_result = fetch_wind(PLEASURE_POINT_SPOT_ID).await.expect("Failed to load wind");
    let conditions_result = fetch_conditions(PLEASURE_POINT_SPOT_ID)
        .await
        .expect("Failed to load conditions");
    let spot_details_result = fetch_spot_details(PLEASURE_POINT_SPOT_ID)
        .await
        .expect("Failed to fetch Spot details");
    let surf_report = SurfReport::new_from_results(
        wave_result,
        tides_result,
        weather_result,
        wind_result,
        conditions_result,
        spot_details_result,
    );
    let data = serde_json::to_string_pretty(&surf_report).expect("Failed to serialize surf report");
    fs::write("tests/data/surf_report_24h.json", data)
        .await
        .expect("Unable to write file");

    // draw the report
    let mut display = SimulatorDisplay::<TriColor>::new(Size::new(800, 480));

    let contents = fs::read_to_string("./tests/data/surf_report_24h.json")
        .await
        .expect("Should have been able to read the file");
    let surf_report = serde_json::from_str(&contents).expect("Failed to parse surf report");

    draw(&mut display, &surf_report).expect("Failed to draw");

    let output_settings = OutputSettingsBuilder::new().scale(1).build();
    let output_image = display.to_grayscale_output_image(&output_settings);
    output_image
        .save_png("tests/data/surf_report_24h.png")
        .expect("Failed to save test image");
}
