use tokio::fs;

use surfboard_scraper::{surf_report::SurfReport, tide::fetch_tides, wave::fetch_waves};

const PLEASURE_POINT_SPOT_ID: &str = "5842041f4e65fad6a7708807";

#[tokio::test]
async fn test_surf_report() {
    let tides_result = fetch_tides(PLEASURE_POINT_SPOT_ID).await.expect("Failed to load tides");
    let wave_result = fetch_waves(PLEASURE_POINT_SPOT_ID).await.expect("Failed to load waves");
    let surf_report = SurfReport::new_from_results(wave_result, tides_result);
    let data = serde_json::to_string(&surf_report).expect("Failed to serialize surf report");
    fs::write("tests/data/surf_report.json", data)
        .await
        .expect("Unable to write file");
}
