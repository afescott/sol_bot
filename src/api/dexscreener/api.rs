use std::sync::Arc;

use reqwest::{header, Client, RequestBuilder};
use url::Url;

use crate::{
    api::{dexscreener::PairResponse, Market},
    error::Result,
};

pub const BASE_URL: &str = "https://api.dexscreener.com/latest/";

/// A [Dexscreener API](https://docs.dexscreener.com/api/reference) HTTP client.
#[derive(Debug)]
pub struct DexClient {
    pub client: Arc<Client>,
    pub url: Url,
    /*     pub dex: Arc<Mutex<Vec<Market>>>, */
}

impl DexClient {
    pub fn new(client: Arc<Client>) -> Self {
        Self {
            url: Url::parse(BASE_URL).unwrap(),
            /*             dex: Arc::new(Mutex::new(Vec::new())), */
            client,
        }
    }

    /// Performs an HTTP `GET` request to the `https://api.dexscreener.com/latest/dex/search/?q=:q` path.
    pub async fn get_token_by_addr(&self, addr: String) -> Result<PairResponse> {
        let path = self.url.join(format!("dex/tokens/{}", addr).as_str());
        println!("{:?}", addr);
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
    let (tx, mut rx1) = tokio::sync::mpsc::channel::<Market>(100000);
    let client = DexClient::new();

    println!(
        "{:?}",
        client
            .get_token_by_addr("2YE4Dmfv2HjwuxXsn9fJy8cHkZKDAk32KVNbak7spump".to_string())
            .await
    );
}

#[tokio::test]
async fn test_retain() {
    let mut asfa = vec![123, 213, 123];

    asfa.retain(|s| {
        {
            if *s == 123 {
                return true;
            }
        };
        false
    });
}
