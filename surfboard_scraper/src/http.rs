use anyhow::{Result, anyhow};
use reqwest::Error;
use serde::de::DeserializeOwned;

pub async fn fetch<T: DeserializeOwned>(url: &str) -> Result<T> {
    let response = reqwest::get(url).await;

    match response {
        Ok(res) => match res.status().is_success() {
            true => {
                let json: Result<T, Error> = res.json().await;
                match json {
                    Ok(json_data) => Ok(json_data),
                    Err(e) => Err(e.into()),
                }
            }
            false => Err(anyhow!("Request failed with status: {}", res.status())),
        },
        Err(err) => Err(err.into()),
    }
}
