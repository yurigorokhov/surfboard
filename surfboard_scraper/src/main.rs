mod common;
mod http;
mod image_data;
mod screen;
mod surf_report_24h;
mod surf_report_week;
mod surfline_types;
mod util;

use anyhow::Result;
use aws_config::meta::region::RegionProviderChain;
use aws_sdk_s3::{Client, types::ObjectCannedAcl};
use std::{collections::HashSet, io::Cursor};
use surfboard_scraper::device_config::Configuration;
use tokio::{
    fs,
    time::{Duration, sleep},
};
use glob::glob;

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

const CONFIG_DIRECTORY: &'static str = "deploy/configs";

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

        // cache output URLs so we don't process the same surf spots over and over
        let mut already_processed_screen_urls: HashSet<String> = HashSet::new();

        // parse config files and upload surf reports
        for entry in glob(format!("{}/*.json", CONFIG_DIRECTORY).as_str()).expect("Failed to read glob pattern") {
            let path = entry?;
            let config: Configuration = serde_json::from_str(fs::read_to_string(&path).await?.as_str())?;
            for screen in config.screens {
                if !already_processed_screen_urls.contains(&screen.url) {
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
                    already_processed_screen_urls.insert(screen.url);
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

            // upload the configs themselves
            let config_str = fs::read(&path).await?;
            upload_bytes(&client, "yurig-public", config_str.as_slice(), path.file_name().unwrap().to_str().unwrap()).await?;
            println!("Uploaded config, bytes: {}", config_str.as_slice().len());
        }
        sleep(Duration::from_secs(3600 * 3)).await;
    }
}
