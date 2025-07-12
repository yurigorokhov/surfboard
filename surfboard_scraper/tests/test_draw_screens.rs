use std::io::Cursor;
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

#[tokio::test]
async fn test_draw_screns_qoi() {
    let config: Configuration = serde_json::from_str(
        fs::read_to_string("deploy/config.json")
            .await
            .expect("Failed ot read config.json")
            .as_str(),
    )
    .expect("Failed to parse configuration");

    for screen in config.screens {
        let mut bytes: Vec<u8> = Vec::new();
        screen
            .draw_to_qoi(&mut Cursor::new(&mut bytes))
            .await
            .expect("Failed to draw qoi");
        assert!(bytes.len() < 1024 * 24)
    }
    if let Some(screen_saver) = config.screen_saver {
        let mut bytes: Vec<u8> = Vec::new();
        screen_saver
            .draw_to_qoi(&mut Cursor::new(&mut bytes))
            .await
            .expect("Failed to draw qoi");
        assert!(bytes.len() < 1024 * 24)
    }
}
