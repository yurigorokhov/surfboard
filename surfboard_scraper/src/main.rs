mod http;
mod surf_report;
mod tide;
mod wave;
mod weather;
mod wind;

use crate::{surf_report::SurfReport, tide::fetch_tides, wave::fetch_waves, weather::fetch_weather, wind::fetch_wind};

const PLEASURE_POINT_SPOT_ID: &str = "5842041f4e65fad6a7708807";

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let tides_result = fetch_tides(PLEASURE_POINT_SPOT_ID).await?;
    let wave_result = fetch_waves(PLEASURE_POINT_SPOT_ID).await?;
    let weather_result = fetch_weather(PLEASURE_POINT_SPOT_ID).await?;
    let wind_result = fetch_wind(PLEASURE_POINT_SPOT_ID).await?;
    let surf_report = SurfReport::new_from_results(wave_result, tides_result, weather_result, wind_result);
    dbg!(&surf_report);
    Ok(())
}
