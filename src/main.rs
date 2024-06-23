use api::dexscreener::{api::DexClient, Pair};
use error::Result;
use repositories::mem::StorageRepo;
pub use reqwest::{self, Client, IntoUrl, Url};
use std::{collections::HashMap, sync::Arc};
use tokio::sync::broadcast;

// Dexscreener API URL (https://docs.dexscreener.com/api/reference).

mod error;
pub mod format;
pub use format::format_addresses;
pub mod api;
pub mod repositories;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let dex_client = DexClient::default();

    let (tx, mut rx1) = broadcast::channel(16);
    let mut rx2 = tx.subscribe();

    let sol_api = api::SolanaRpc::new(tx);
    // task 1: access webhook. use channel to send new tokens
    let handle = std::thread::spawn(move || {
        sol_api.get_transactions();
    });

    let mut hash_map: HashMap<String, Pair> = HashMap::new();

    //task 1 check dex_screener api
    tokio::spawn(async move {
        while let Ok(res) = rx1.recv().await {
            println!("fix market token address val received:{:?}", res);

            let pair = dex_client
                .get_token_by_addr(res.token_address.to_string())
                .await
                .unwrap();

            let pair = pair.pairs;

            if let Some(pair) = pair {
                if let Some(pair) = pair.first() {
                    hash_map.insert(res.token_address.to_string(), pair.to_owned());
                }
            }
        }
    });

    //task 2 check token sniffer etc
    tokio::spawn(async move {
        while let Ok(res) = rx2.recv().await {
            //check for legit tokenomics
        }
    });

    /* //task 2: receive new tokens.  search via dexclient & other sources
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
    }); */

    /*     let _r = tokio::join!(r, s); */
}
