mod conditions;
mod http;
mod spot_details;
mod surf_report;
mod tide;
mod wave;
mod weather;
mod wind;

use aws_config::meta::region::RegionProviderChain;
use aws_sdk_s3::{types::ObjectCannedAcl, Client};
use tokio::time::{sleep, Duration};

use crate::{
    conditions::fetch_conditions, spot_details::fetch_spot_details, surf_report::SurfReport, tide::fetch_tides,
    wave::fetch_waves, weather::fetch_weather, wind::fetch_wind,
};

const PLEASURE_POINT_SPOT_ID: &str = "5842041f4e65fad6a7708807";

pub async fn upload_object(
    client: &aws_sdk_s3::Client,
    bucket_name: &str,
    content: &str,
    key: &str,
) -> Result<aws_sdk_s3::operation::put_object::PutObjectOutput, Box<dyn std::error::Error>> {
    Ok(client
        .put_object()
        .bucket(bucket_name)
        .key(key)
        .body(aws_sdk_s3::primitives::ByteStream::from(content.as_bytes().to_vec()))
        .acl(ObjectCannedAcl::PublicRead)
        .send()
        .await
        .unwrap())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // TODO: graceful shutdown
    loop {
        let tides_result = fetch_tides(PLEASURE_POINT_SPOT_ID).await?;
        let wave_result = fetch_waves(PLEASURE_POINT_SPOT_ID).await?;
        let weather_result = fetch_weather(PLEASURE_POINT_SPOT_ID).await?;
        let wind_result = fetch_wind(PLEASURE_POINT_SPOT_ID).await?;
        let conditions_result = fetch_conditions(PLEASURE_POINT_SPOT_ID).await?;
        let spot_details = fetch_spot_details(PLEASURE_POINT_SPOT_ID).await?;
        let surf_report = SurfReport::new_from_results(
            wave_result,
            tides_result,
            weather_result,
            wind_result,
            conditions_result,
            spot_details,
        );
        let report_serialized = serde_json::to_string(&surf_report)?;

        let region_provider = RegionProviderChain::default_provider().or_else("us-east-1");
        let config = aws_config::from_env()
            .region(region_provider)
            .profile_name("yurigorokhov")
            .load()
            .await;
        let client = Client::new(&config);
        upload_object(&client, "yurig-public", &report_serialized, "surf_report.json").await?;

        sleep(Duration::from_secs(3600 * 3)).await;
    }
    Ok(())
}
