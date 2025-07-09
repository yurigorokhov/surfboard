use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Configuration {
    pub screens: Vec<ScreenConfiguration>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ScreenConfiguration {
    pub key: String,
    pub url: String,
}
