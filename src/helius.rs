use reqwest::Client;
use url::Url;

pub const HELIUS_URL: &str = "https://api.helius.xyz/v0/INSERT_SOMETHING/{webhookID}";

pub struct HeliusClient {
    client: Client,
    url: Url,
}

impl Default for HeliusClient {
    fn default() -> Self {
        Self {
            client: Client::new(),
            url: Url::parse(HELIUS_URL).unwrap(),
        }
    }
}

impl HeliusClient {}
