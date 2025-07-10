use anyhow::{Result, anyhow};
use url::Url;

pub fn parse_s3_url(s3_url: &str) -> Result<(String, String)> {
    let url = Url::parse(s3_url)?;

    let mut bucket = url.host_str().ok_or(anyhow!("Missing bucket name"))?.to_string();

    if bucket.contains(".amazonaws.com") {
        bucket = bucket.split(".").nth(0).unwrap().to_string();
    }

    let key = url.path().strip_prefix('/').unwrap_or("").to_string();

    Ok((bucket, key))
}
