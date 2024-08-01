use solana_sdk::pubkey::{self, Pubkey};
use std::collections::HashSet;

use crate::api::rugcheck::api::RugCheckClient;
use api::{jupiter::JupiterSwapApiClient, Market, TokenRiskMetaData};
use error::Result;
use repositories::dex_screener::DexMem;
pub use reqwest::{self, Client, IntoUrl, Url};

pub mod api;
mod error;
pub mod format;
pub mod repositories;

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();

    let (tx, rx1) = tokio::sync::broadcast::channel::<Market>(100000);
    let rx2 = tx.subscribe();

    let (tx_token_data, mut rx_token_data) = tokio::sync::mpsc::channel::<TokenRiskMetaData>(10000);

    let sol_api = api::SolanaRpc::new(tx);

    // task 1: subscribe to solana webhook webhook. use channel to send new tokens
    let handle = tokio::task::spawn(async move {
        println!("starting sol webhook");
        loop {
            sol_api.get_transactions().await;
        }
    });

    let client = reqwest::Client::new();

    let xyz_client = RugCheckClient::new(client.clone(), tx_token_data.clone(), rx2);
    let dex_mem = DexMem::new(tx_token_data, rx1, client.clone());

    // task 2: listen for incoming tokens and verify tokenomics via dex_screener
    let handle_2 = tokio::spawn(async move {
        // while let Some(s) = rx1.recv().await {
        dex_mem.loop_awaiting_liquidity_tokens().await;
    });

    /*      task 3: use xyz to ensure token "legitimacy" */
    let handle_3 = tokio::spawn(async move {
        //check for memory
        xyz_client.loop_token_reliability_info().await;
    });

    let jup_api = JupiterSwapApiClient::default();

    let handle_purchase_token = tokio::spawn(async move {
        //tokens are only received if they meet the requirements
        let mut tokens: HashSet<Pubkey> = HashSet::new();
        // let mut temp_hashmap: HashMap<String, bool> = HashMap::new(); //if this is true
        while let Some(token_meta_data) = rx_token_data.recv().await {
            match token_meta_data {
                TokenRiskMetaData::DexScreenerResponse(dex) => {
                    if tokens.get(&dex).is_some() {
                        // buy jup_api
                        if let Err((a)) = jup_api.buy(dex).await {
                        } else {
                            tokens.remove(&dex);
                        }
                    } else {
                        tokens.insert(dex);
                    }
                }
                TokenRiskMetaData::XyzResponse(xyz) => {}
            }
        }
    });

    tokio::join!(handle_2, handle, handle_3, handle_purchase_token);

    Ok(())
}
