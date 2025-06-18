mod tide;
use tide::fetch_tides;

use crate::{surf_report::SurfReport, wave::fetch_waves};

mod http;
mod surf_report;
mod wave;

const PLEASURE_POINT_SPOT_ID: &str = "5842041f4e65fad6a7708807";

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let tides_result = fetch_tides(PLEASURE_POINT_SPOT_ID).await?;
    let wave_result = fetch_waves(PLEASURE_POINT_SPOT_ID).await?;
    let surf_report = SurfReport::new_from_results(wave_result, tides_result);
    dbg!(&surf_report);
    Ok(())
}

//TODO: WEATHER curl https://services.surfline.com/kbyg/spots/forecasts/weather\?spotId\=5842041f4e65fad6a7708906\&days\=1\&intervalHours\=1
//TODO: WIND curl https://services.surfline.com/kbyg/spots/forecasts/wind\?spotId\=5842041f4e65fad6a7708906\&days\=1\&intervalHours\=1
