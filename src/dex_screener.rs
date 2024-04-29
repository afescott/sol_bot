use reqwest::Client;
use url::Url;

use crate::pair::Response;

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
    /// Performs an HTTP `GET` request to the `/dex/pairs/{chain_id}/{pair_addresses}` path.
    pub async fn pairs(
        &self,
        chain_id: impl Display,
        pair_addresses: impl IntoIterator<Item = impl AsRef<str>>,
    ) -> error::Result<Response> {
        let addresses = format_addresses(pair_addresses).unwrap();
        let path = format!("dex/pairs/{chain_id}/{addresses}");

        Ok(self
            .client
            .get(BASE_URL.to_owned() + &path)
            .send()
            .await?
            .json::<Response>()
            .await?)
    }
}
