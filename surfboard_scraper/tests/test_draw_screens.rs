use tokio::fs;

use surfboard_scraper::device_config::Configuration;

#[tokio::test]
async fn test_draw_screns() {
    let config: Configuration = serde_json::from_str(
        fs::read_to_string("deploy/config.json")
            .await
            .expect("Failed ot read config.json")
            .as_str(),
    )
    .expect("Failed to parse configuration");

    for screen in config.screens {
        screen
            .draw_to_png(&format!("tests/data/{}.png", screen.id))
            .await
            .expect("Failed to draw image");
    }
    if let Some(screen_saver) = config.screen_saver {
        screen_saver
            .draw_to_png(&format!("tests/data/{}.png", screen_saver.id))
            .await
            .expect("Failed to draw image");
    }
}
