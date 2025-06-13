use reqwless::client::HttpClient;
// use reqwless::client::TlsConfig;
// use reqwless::client::TlsVerify;
use reqwless::request::Method;
use reqwless::Error as ReqlessError;

use heapless::Vec;

/// Response size
const RESPONSE_SIZE: usize = 4096;
// const TLS_DATA_BUFFER_SIZE: usize = 2048; // 16640

