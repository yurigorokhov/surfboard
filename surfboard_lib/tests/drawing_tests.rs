use embedded_graphics::prelude::Size;
use embedded_graphics_simulator::{OutputSettingsBuilder, SimulatorDisplay};
use epd_waveshare::color::TriColor;
use std::fs;
use surfboard_lib::{
    data::{ProgramState, SurfReportResponse},
    draw::DisplayAction,
};

#[tokio::test]
async fn test_surf_report() {
    let mut display = SimulatorDisplay::<TriColor>::new(Size::new(800, 480));

    let contents =
        fs::read_to_string("tests/data/surf_report_response.json").expect("Should have been able to read the file");
    let (data, _remainder) = serde_json_core::from_str::<SurfReportResponse>(contents.as_str()).unwrap();
    let mut program_state = ProgramState::default();
    program_state
        .update_from_surf_report(data)
        .expect("Failed to update program state");

    DisplayAction::DisplaySurfReport
        .draw(&mut display, &program_state)
        .expect("Failed to draw surfreport");

    let output_settings = OutputSettingsBuilder::new().scale(1).build();
    let output_image = display.to_grayscale_output_image(&output_settings);
    output_image
        .save_png("tests/screenshots/surf_report.png")
        .expect("Failed to save test image");
}
