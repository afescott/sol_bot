use std::{fmt::Display, sync::Arc};

use error::{PairResponse, Result};
use repositories::models::Token;
pub use reqwest::{self, Client, IntoUrl, Url};
use reqwest::{header, RequestBuilder};

// Dexscreener API URL (https://docs.dexscreener.com/api/reference).

mod error;
pub mod format;
pub use format::format_addresses;
use serde::de::DeserializeOwned;

use crate::api::webhook::webhook_messages;
pub mod api;
pub mod pair;
pub mod repositories;

pub const BASE_URL: &str = "https://api.dexscreener.com/latest/";

#[tokio::main]
async fn main() {
    let mut tokens: Vec<Token> = Vec::new();

    let (tx, mut rx) = tokio::sync::mpsc::channel(10000);

    let client = DexClient::default();

    // thread 1: access webhook. use channel to send new tokens
    let r = tokio::spawn(async move {
        webhook_messages(tx).await;
    });

    //thread 2: receive new tokens.  search via dexclient & other sources
    let s = tokio::spawn(async move {
        while let Some(i) = rx.recv().await {
            for ele in i {
                println!("{:?}", ele);
                if !tokens.contains(&ele) {
                    tokens.push(ele.clone());
                }
            }

            println!("{:?}", tokens)
        }
    });

    let _r = tokio::join!(r, s);
}

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
    }
}

#[tokio::test]
async fn test_get_data() {
    let client = DexClient::default();
    let pair_addresses = [
        "0x7213a321F1855CF1779f42c0CD85d3D95291D34C",
        "0x16b9a82891338f9ba80e2d6970fdda79d1eb0dae",
    ];
    let result = client.pairs("bsc", pair_addresses).await.unwrap();
    println!("{:?}", result)
}
