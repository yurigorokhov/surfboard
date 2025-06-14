use defmt::*;
use embassy_net::{
    dns,
    tcp::client::{TcpClient, TcpClientState},
    Stack,
};
use embassy_rp::clocks::RoscRng;
use reqwless::{
    client::{HttpClient, TlsConfig, TlsVerify},
    request::Method,
};
use serde::de::DeserializeOwned;
use surfboard_lib::data::HttpDataProvider;

pub struct HttpClientProvider {
    stack: Stack<'static>,
}

impl HttpClientProvider {
    pub fn new(stack: Stack<'static>) -> Self {
        HttpClientProvider { stack }
    }
}

impl HttpDataProvider for HttpClientProvider {
    async fn get_as_json<'a, DataType: DeserializeOwned>(
        &'a self,
        url: &'a str,
        buffer: &'a mut [u8],
    ) -> Option<DataType> {
        // let mut rng = RoscRng;
        // let seed = rng.next_u64();
        let seed: u64 = 1337;

        let mut tls_read_buffer = [0; 16640];
        let mut tls_write_buffer = [0; 16640];

        let client_state = TcpClientState::<1, 1024, 1024>::new();
        let tcp_client = TcpClient::new(self.stack, &client_state);
        let dns_client = dns::DnsSocket::new(self.stack);
        let tls_config = TlsConfig::new(seed, &mut tls_read_buffer, &mut tls_write_buffer, TlsVerify::None);

        let mut http_client = HttpClient::new_with_tls(&tcp_client, &dns_client, tls_config);

        let mut request = match http_client.request(Method::GET, url).await {
            Ok(req) => Some(req),
            Err(e) => {
                error!("Failed to make HTTP request: {:?}", Debug2Format(&e));
                None
            }
        }?;

        debug!("request created");
        let response = match request.send(buffer).await {
            Ok(resp) => Some(resp),
            Err(e) => {
                error!("Request error: {:?}", Debug2Format(&e));
                None
            }
        }?;
        debug!("response code: {}", response.status.0);
        let body = response
            .body()
            .read_to_end()
            .await
            .expect("Failed to get response body");
        let (data, _remainder) = serde_json_core::from_slice::<DataType>(body).unwrap();
        Some(data)
    }
}
