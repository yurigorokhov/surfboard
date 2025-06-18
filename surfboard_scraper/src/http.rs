use reqwest::Error;
use serde::de::DeserializeOwned;

pub async fn fetch<T: DeserializeOwned>(url: &str) -> Result<T, Box<dyn std::error::Error>> {
    let response = reqwest::get(url).await;

    match response {
        Ok(res) => match res.status().is_success() {
            true => {
                let json: Result<T, Error> = res.json().await;
                match json {
                    Ok(json_data) => Ok(json_data),
                    Err(e) => Err(Box::new(e)),
                }
            }
            false => Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!("Request failed with status: {}", res.status()),
            ))),
        },
        Err(err) => Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::Other,
            format!("Request failed : {}", err.to_string()),
        ))),
    }
}
