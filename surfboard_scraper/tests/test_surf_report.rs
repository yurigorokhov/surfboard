use tokio::fs;

use surfboard_scraper::{
    conditions::fetch_conditions, spot_details::fetch_spot_details, surf_report::SurfReport, tide::fetch_tides,
    wave::fetch_waves, weather::fetch_weather, wind::fetch_wind,
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
    fs::write("tests/data/surf_report.json", data)
        .await
        .expect("Unable to write file");
}
