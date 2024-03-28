use std::fmt::Display;

use error::{PairResponse, Result};
pub use reqwest::{self, Client, IntoUrl, Url};
use reqwest::{header, RequestBuilder};

// Dexscreener API URL (https://docs.dexscreener.com/api/reference).

mod error;
pub mod format;
pub use format::format_addresses;
use serde::de::DeserializeOwned;
pub mod pair;

pub const BASE_URL: &str = "https://api.dexscreener.com/latest/";

fn main() {}

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
    /// Instantiate a new client with the [base URL][BASE_URL].
    /* pub fn new() -> Self {
        Self::with_url(BASE_URL).unwrap()
    } */

    /// Instantiate a new client with the provided URL.
    /* pub fn with_url(url: impl IntoUrl) -> Result<Self> {
        Self::with_url_and_client(url, Client::new())
    } */

    /*
    /// Instantiate a new client with the provided URL and reqwest client.
    pub fn with_url_and_client(url: impl IntoUrl, client: Client) -> Result<Self> {
        Ok(Self {
            client,
            url: url.into_url()?,
        })
    } */

    async fn get_pair<T: DeserializeOwned>(&self, path: &str) -> Result<String> {
        /*         Ok( */
        Ok(self
            ._get(path)?
            .send()
            .await?
            .error_for_status()?
            .json::<String>()
            .await
            .unwrap())
        /*             .await?) */

        /*         todo!() */
    }

    fn _get(&self, path: &str) -> Result<RequestBuilder> {
        let url = self.url.join(path)?;
        Ok(self
            .client
            .get(url)
            .header(header::ACCEPT, "application/json"))
    }

    /// Performs an HTTP `GET` request to the `/dex/pairs/{chain_id}/{pair_addresses}` path.
    pub async fn pairs(
        &self,
        chain_id: impl Display,
        pair_addresses: impl IntoIterator<Item = impl AsRef<str>>,
    ) -> Result<String> {
        let addresses = format_addresses(pair_addresses)?;
        let path = format!("dex/pairs/{chain_id}/{addresses}");
        self.get_pair::<String>(&path).await

        /*         Ok("asfaf".to_string()) */
    }
}

/* mod test {
use crate::DexClient; */

#[tokio::test]
async fn test_get_data() {
    let client = DexClient::default();

    /*     let r = client.client.get(client.url).send().await; */

    let pair_addresses = [
        "0x7213a321F1855CF1779f42c0CD85d3D95291D34C",
        "0x16b9a82891338f9ba80e2d6970fdda79d1eb0dae",
    ];
    let result = client.pairs("bsc", pair_addresses).await.unwrap();
    /* .pairs
    .unwrap(); */
    /*     assert_eq!(result.len(), 2); */
    println!("{:?}", result)
}
/* } */
