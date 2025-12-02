use std::fs;

use surfboard_scraper::device_config::Configuration;
use glob::glob;


const CONFIG_DIRECTORY: &'static str = "deploy/configs";


#[tokio::test]
async fn test_config_parsing() {

    for entry in glob(format!("{}/*.json", CONFIG_DIRECTORY).as_str()).expect("Failed to read glob pattern") {
            let path = entry.expect("Failed");

        let contents =
            fs::read_to_string(&path).expect("Should have been able to parse the configuration file");
        let _data = serde_json::from_str::<Configuration>(contents.as_str()).unwrap();
    }
}
