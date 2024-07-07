use std::sync::Arc;
use tokio::time::Instant;

use api::Market;
use error::Result;
use repositories::dex_screener::DexMem;
pub use reqwest::{self, Client, IntoUrl, Url};
// Dexscreener API URL (https://docs.dexscreener.com/api/reference).

mod error;
pub mod format;
pub use format::format_addresses;

use crate::api::rugcheck::api::RugCheckClient;
pub mod api;
pub mod repositories;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let (tx, rx1) = tokio::sync::broadcast::channel::<Market>(100000);
    let mut rx2 = tx.subscribe();

    // task 1: subscribe to solana webhook webhook. use channel to send new tokens
    let handle = tokio::task::spawn(async move {
        println!("starting sol webhook");

        let sol_api = api::SolanaRpc::new(tx);
        sol_api.get_transactions();
    });

    let client = Arc::new(Client::new());

    let client_1 = client.clone();
    let client_2 = client.clone();

    let xyz_client = RugCheckClient::new(client_2);
    let dex_mem = DexMem::new(rx1, client_1);

    // task 2: listen for incoming tokens and verify tokenomics via dex_screener
    let handle_2 = tokio::spawn(async move {
        dex_mem.loop_awaiting_liquidity_tokens().await;
    });

    // task 3: use xyz to ensure token "legitimacy"
    let handle_3 = tokio::spawn(async move {
        //check for memory
        while let Ok(s) = rx2.recv().await {
            /*             if let Ok(s) = result { */
            let now = Instant::now();
            println!("trying xyz");
            /*                 dex_client.loop_awaiting_liquidity_tokens(s).await; */
            let r = xyz_client
                .get_token_reliability_info(s.token_address.to_string())
                .await;

            if let Ok(done) = r {
                println!("xyz: {:?}", done);
            } else {
                println!("xyz failed: {:?}", s.token_address.to_string());
            }
            println!("xyz elasped: {:.3?}", now.elapsed());
        }
    });

    /*     tokio::spawn(async move {}); */

    tokio::join!(handle_2, handle, handle_3);
}
