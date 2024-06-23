use reqwest::{header, Client, RequestBuilder};
use serde::de::DeserializeOwned;
use url::Url;

use crate::{
    api::dexscreener::{Pair, PairResponse},
    error::Result,
};

pub const BASE_URL: &str = "https://api.dexscreener.com/latest/";

/// A [Dexscreener API](https://docs.dexscreener.com/api/reference) HTTP client.
#[derive(Clone, Debug)]
pub struct DexClient {
    pub client: Client,
    pub url: Url,
}

impl Default for DexClient {
    fn default() -> Self {
        Self {
            client: Client::new(),
            url: Url::parse(BASE_URL).unwrap(),
        }
    }
}

impl DexClient {
    /// Performs an HTTP `GET` request to the `https://api.dexscreener.com/latest/dex/search/?q=:q` path.
    pub async fn get_token_by_addr(&self, addr: String) -> Result<PairResponse> {
        let path = self.url.join(format!("dex/tokens/{}", addr).as_str());

        let r = self
            .client
            .get(path.unwrap())
            .header(header::ACCEPT, "application/json")
            .send()
            .await
            .unwrap();

        Ok(r.json::<PairResponse>().await?)
    }
}

#[tokio::test]
async fn test_get_token() {
    let client = DexClient::default();

    println!(
        "{:?}",
        client
            .get_token_by_addr("5s2VrJWrCncxSY6io7xR5JNzESWhfQfkeihhkf5YLuV3".to_string())
            .await
    );
}
