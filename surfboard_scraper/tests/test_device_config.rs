use std::fs;

use surfboard_scraper::device_config::Configuration;

#[tokio::test]
async fn test_config_parsing() {
    let contents =
        fs::read_to_string("deploy/config.json").expect("Should have been able to parse the configuration file");
    let _data = serde_json::from_str::<Configuration>(contents.as_str()).unwrap();
}
