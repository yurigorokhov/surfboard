use heapless::{String, Vec};

use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};

#[trait_variant::make(HttpService: Send)]
pub trait HttpDataProvider {
    async fn get_as_json<'a, DataType: DeserializeOwned>(
        &'a self,
        url: &'a str,
        buffer: &'a mut [u8],
    ) -> Option<DataType>;
}

/****** Tide predictions ******/

#[derive(Deserialize, Serialize)]
pub struct TidePredictionsDataPoint {
    pub t: String<16>,
    pub v: String<8>,
}

#[derive(Deserialize, Serialize)]
pub struct TidePredictions {
    pub predictions: Vec<TidePredictionsDataPoint, 24>,
}

pub async fn tide_data<'a, T: HttpDataProvider>(client: &'a T, buffer: &'a mut [u8]) -> Option<TidePredictions> {
    let url = "https://api.tidesandcurrents.noaa.gov/api/prod/datagetter?date=today&station=9413450&product=predictions&datum=STND&time_zone=lst&interval=h&units=english&format=json";
    let data = client.get_as_json::<TidePredictions>(url, buffer).await;
    data
}

#[cfg(test)]
mod tests {
    use super::*;
    use reqwest;

    struct TestDataProvider {}

    impl HttpDataProvider for TestDataProvider {
        async fn get_as_json<'a, DataType: DeserializeOwned>(
            &'a self,
            url: &'a str,
            buffer: &'a mut [u8],
        ) -> Option<DataType> {
            let response_str = reqwest::get(url).await.ok()?.text().await.ok()?;
            let response_bytes = response_str.as_bytes();
            buffer[0..response_bytes.len()].copy_from_slice(response_bytes);
            let (data, _remainder) = serde_json_core::from_slice::<DataType>(&buffer[0..response_bytes.len()]).unwrap();
            Some(data)
        }
    }

    #[tokio::test]
    async fn test_get_tide_data() {
        let http_provider = TestDataProvider {};
        let mut buffer = [0_u8; 4096];
        let data = tide_data(&http_provider, &mut buffer).await;
        assert!(data.is_some());
        assert!(data.unwrap().predictions.len() > 0);
    }
}
