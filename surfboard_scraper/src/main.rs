mod http;
mod image_data;
mod surf_report_24h;

use crate::surf_report_24h::draw::draw;
use aws_config::meta::region::RegionProviderChain;
use aws_sdk_s3::{Client, types::ObjectCannedAcl};
use embedded_graphics::prelude::Size;
use embedded_graphics_simulator::{OutputSettingsBuilder, SimulatorDisplay};
use epd_waveshare::color::TriColor;
use std::io::Cursor;
use tokio::time::{Duration, sleep};

use crate::{
    surf_report_24h::conditions::fetch_conditions, surf_report_24h::spot_details::fetch_spot_details,
    surf_report_24h::surf_report::SurfReport, surf_report_24h::tide::fetch_tides, surf_report_24h::wave::fetch_waves,
    surf_report_24h::weather::fetch_weather, surf_report_24h::wind::fetch_wind,
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

pub async fn upload_bytes(
    client: &aws_sdk_s3::Client,
    bucket_name: &str,
    content: &[u8],
    key: &str,
) -> Result<aws_sdk_s3::operation::put_object::PutObjectOutput, Box<dyn std::error::Error>> {
    Ok(client
        .put_object()
        .bucket(bucket_name)
        .key(key)
        .body(aws_sdk_s3::primitives::ByteStream::from(content.to_vec()))
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

        // draw surf report as image
        let mut display = SimulatorDisplay::<TriColor>::new(Size::new(800, 480));
        draw(&mut display, &surf_report)?;
        let output_settings = OutputSettingsBuilder::new().scale(1).build();
        let output_image = display.to_rgb_output_image(&output_settings);
        let image_buffer = output_image.as_image_buffer();
        let mut bytes: Vec<u8> = Vec::new();
        image_buffer
            .write_to(&mut Cursor::new(&mut bytes), image::ImageFormat::Qoi)
            .unwrap();

        // upload surf report
        let region_provider = RegionProviderChain::default_provider().or_else("us-east-1");
        let config = aws_config::from_env()
            .region(region_provider)
            .profile_name("yurigorokhov")
            .load()
            .await;
        let client = Client::new(&config);
        upload_bytes(&client, "yurig-public", &bytes, "surf_report.qoi").await?;
        println!("Uploaded");

        sleep(Duration::from_secs(3600 * 3)).await;
    }
    Ok(())
}
