use embedded_nal_async::{Dns, TcpConnect};
use reqwless::client::HttpClient;
use reqwless::request::Method;

use heapless::Vec;

/// Response size
const RESPONSE_SIZE: usize = 4096;
// const TLS_DATA_BUFFER_SIZE: usize = 2048; // 16640

pub async fn tide_data<'a, T, D>(client: &mut HttpClient<'a, T, D>) -> Vec<u8, RESPONSE_SIZE>
where
    T: TcpConnect + 'a,
    D: Dns + 'a,
{
    let url = "https://api.tidesandcurrents.noaa.gov/api/prod/datagetter?date=today&station=9413450&product=predictions&datum=STND&time_zone=lst&interval=h&units=english&format=json";
    let mut buffer = [0_u8; RESPONSE_SIZE];
    let mut request = client.request(Method::GET, url).await.expect("HTTP ERROR");
    let response = request.send(&mut buffer).await.expect("HTTP ERROR");
    let buffer = response.body().read_to_end().await.expect("READ BODY");
    let output = Vec::<u8, RESPONSE_SIZE>::from_slice(buffer).expect("READ BODY");
    output
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_tide_data() {
        let mut client = HttpClient::new(TokioTcp, StaticDns);
    }
}
