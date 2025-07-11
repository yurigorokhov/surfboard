mod http;
mod image_data;
mod photo;
mod screen;
mod surf_report_24h;
mod surfline_types;
mod util;

use anyhow::Result;
use aws_config::meta::region::RegionProviderChain;
use aws_sdk_s3::{Client, types::ObjectCannedAcl};
use std::io::Cursor;
use surfboard_scraper::device_config::Configuration;
use tokio::{
    fs,
    time::{Duration, sleep},
};

use crate::util::parse_s3_url;

pub async fn upload_object(
    client: &aws_sdk_s3::Client,
    bucket_name: &str,
    content: &str,
    key: &str,
) -> Result<aws_sdk_s3::operation::put_object::PutObjectOutput> {
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
) -> Result<aws_sdk_s3::operation::put_object::PutObjectOutput> {
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

const CONFIG_PATH: &'static str = "deploy/config.json";

#[tokio::main]
async fn main() -> Result<()> {
    // TODO: graceful shutdown

    // initialize AWS S3 client
    let region_provider = RegionProviderChain::default_provider().or_else("us-east-1");
    let config = aws_config::from_env()
        .region(region_provider)
        .profile_name("yurigorokhov")
        .load()
        .await;
    let client = Client::new(&config);

    loop {
        // read config file
        let config: Configuration = serde_json::from_str(fs::read_to_string(CONFIG_PATH).await?.as_str())?;
        for screen in config.screens {
            let mut bytes: Vec<u8> = Vec::new();
            screen.draw_to_qoi(&mut Cursor::new(&mut bytes)).await?;

            // upload screen image
            match parse_s3_url(&screen.url) {
                Ok((bucket, path)) => {
                    upload_bytes(&client, &bucket, &bytes, &path).await?;
                    println!("Uploaded: {} bytes: {}", screen.url, &bytes.len());
                }
                Err(e) => {
                    println!("Error: {:#?}", e);
                }
            }
        }

        // upload screensaver
        if let Some(screen_saver) = config.screen_saver {
            let mut bytes: Vec<u8> = Vec::new();
            screen_saver.draw_to_qoi(&mut Cursor::new(&mut bytes)).await?;
            match parse_s3_url(&screen_saver.url) {
                Ok((bucket, path)) => {
                    upload_bytes(&client, &bucket, &bytes, &path).await?;
                    println!("Uploaded: {} bytes: {}", screen_saver.url, &bytes.len());
                }
                Err(e) => {
                    println!("Error: {:#?}", e);
                }
            }
        }

        let config_str = fs::read(CONFIG_PATH).await?;
        upload_bytes(&client, "yurig-public", config_str.as_slice(), "config.json").await?;
        println!("Uploaded config, bytes: {}", config_str.as_slice().len());

        sleep(Duration::from_secs(3600 * 3)).await;
    }
}
