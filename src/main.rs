use std::sync::Arc;

use api::discord::DiscordParser;
use error::Result;
use repositories::{mem::StorageRepo, models::TokenFinal};
pub use reqwest::{self, Client, IntoUrl, Url};
use reqwest::{header, RequestBuilder};

// Dexscreener API URL (https://docs.dexscreener.com/api/reference).

mod error;
pub mod format;
pub use format::format_addresses;
use serde::de::DeserializeOwned;

use crate::{
    api::webhook::webhook_messages,
    repositories::models::{Pairs, TokenType},
};

pub mod api;
pub mod pair;
pub mod repositories;
mod util;

pub const BASE_URL: &str = "https://api.dexscreener.com/latest/";

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    let storage_enhanced_transactions = Arc::new(StorageRepo::<TokenFinal>::new());

    let time = chrono::DateTime::from_timestamp_millis(1712860625000);

    let mut tokens: Vec<TokenType> = Vec::new();

    let (tx, mut rx) = tokio::sync::mpsc::channel(10000);

    let dex_client = DexClient::default();

    // thread 1: access webhook. use channel to send new tokens
    let r = tokio::spawn(async move {
        webhook_messages(tx).await;
    });

    //thread 2: receive new tokens.  search via dexclient & other sources
    let s = tokio::spawn(async move {
        let filter = DiscordParser::new();

        let client = Client::new();
        while let Some(i) = rx.recv().await {
            for ele in i {
                let results = dex_client.search(ele).await;

                // match ele {
                //     TokenType::Id(id) => todo!(),
                //     TokenType::Name(name) => todo!(),
                //     TokenType::Pairs(pairs) => {
                //         if pairs.len() > 0 {
                //             // filter.get_transaction_info(&client, values);
                //         }
                //         todo!()
                //     }
                // }

                // if !tokens.contains(&ele) {
                //     tokens.push(ele.clone());
                // token_type.process()
                // let results = dex_client.search(ele).await;

                /* match results {
                    Ok(pairs) => pairs_filter(pairs).await,
                    Err(err) => println!("{:?}", err),
                } */
                // }
            }
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
    fn _get(&self, path: &str) -> Result<RequestBuilder> {
        let url = self.url.join(path)?;
        Ok(self
            .client
            .get(url)
            .header(header::ACCEPT, "application/json"))
    }

    /// Performs an HTTP `GET` request to the `https://api.dexscreener.com/latest/dex/search/?q=:q` path.
    pub async fn search(&self, token: TokenType) -> Result<Pairs> {
        let token = match token {
            TokenType::Id(id) => id,
            TokenType::Name(name) => name,
            TokenType::Pairs(pairs) => unimplemented!(),
        };

        let path = self.url.join(format!("dex/search?q={}", token).as_str());

        let r = self
            .client
            .get(path.unwrap())
            .header(header::ACCEPT, "application/json")
            .send()
            .await?;

        Ok(r.json::<Pairs>().await?)
    }

    async fn get_token<T: DeserializeOwned>(&self, path: &str) -> Result<String> {
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
}
