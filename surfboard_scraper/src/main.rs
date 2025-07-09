mod http;
mod image_data;
mod surf_report_24h;
mod surfline_types;

use anyhow::Result;
use aws_config::meta::region::RegionProviderChain;
use aws_sdk_s3::{Client, types::ObjectCannedAcl};
use std::io::Cursor;
use tokio::time::{Duration, sleep};

use crate::surf_report_24h::surf_report::SurfReport24H;

const PLEASURE_POINT_SPOT_ID: &str = "5842041f4e65fad6a7708807";

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
        // draw surf report as image
        let mut bytes: Vec<u8> = Vec::new();
        SurfReport24H::fetch_latest(PLEASURE_POINT_SPOT_ID)
            .await?
            .draw_to_qoi(&mut Cursor::new(&mut bytes))?;

        // upload surf report image
        upload_bytes(&client, "yurig-public", &bytes, "surf_report.qoi").await?;
        println!("Uploaded");

        sleep(Duration::from_secs(3600 * 3)).await;
    }
}
