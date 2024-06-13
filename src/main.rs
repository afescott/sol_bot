use api::dexscreener::api::DexClient;
use error::Result;
use repositories::mem::StorageRepo;
pub use reqwest::{self, Client, IntoUrl, Url};
use std::sync::Arc;

// Dexscreener API URL (https://docs.dexscreener.com/api/reference).

mod error;
pub mod format;
pub use format::format_addresses;
pub mod api;
pub mod repositories;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    /*     let storage_enhanced_transactions = Arc::new(StorageRepo::<TokenFinal>::new()); */

    /*     let (tx, mut rx) = tokio::sync::mpsc::channel(10000); */

    let dex_client = DexClient::default();

    // task 1: access webhook. use channel to send new tokens
    let r = tokio::spawn(async move { /*         webhook_messages(tx).await; */ });

    //task 2: receive new tokens.  search via dexclient & other sources
    let s = tokio::spawn(async move {
        let client = Client::new();
        /* while let Some(i) = rx.recv().await {
            for ele in i {
                let results = dex_client.search(ele).await;

                match results {
                    Ok(pairs) => pairs_filter(pairs).await,
                    Err(err) => println!("{:?}", err),
                }
                // }
            }
        } */
    });

    let _r = tokio::join!(r, s);
}
