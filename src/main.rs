use solana_sdk::pubkey::{self, Pubkey};
use std::{
    collections::HashSet,
    env,
    sync::{Arc, Mutex},
};
use tokio::sync::mpsc::Receiver;

use crate::{
    api::{dexscreener::api::DexClient, rugcheck::api::RugCheckClient},
    repositories::dex_screener::{loop_yet_to_dexscreener, run},
};
use api::{jupiter::JupiterSwapApiClient, Market, TokenRiskMetaData};
use error::Result;
pub use reqwest::{self, Client, IntoUrl, Url};

pub mod api;
mod args;
mod error;
pub mod format;
pub mod repositories;

#[derive(Parser)]
struct Args {
    name: String,
}

#[tokio::main]
async fn main() -> Result<()> {
    /*     let args: Args = Args::parse(); */
    tracing_subscriber::fmt::init();

    let (tx, rx1) = tokio::sync::broadcast::channel::<Market>(50);
    let rx2 = tx.subscribe();

    let (tx_token_data, mut rx_token_data) = tokio::sync::mpsc::channel::<TokenRiskMetaData>(50);

    let sol_api = api::SolanaRpc::new(tx);

    // task 1: subscribe to solana webhook webhook. use channel to send new tokens
    let handle = tokio::task::spawn(async move {
        println!("starting sol webhook");
        loop {
            sol_api.get_transactions().await;
        }
    });

    let xyz_client = RugCheckClient::new(tx_token_data.clone(), rx2);
    /*
    /* let mut dex_mem:
    /*     Arc<Mutex< */
        DexMem =
            DexMem::new(tx_token_data, rx1, client.clone()); */
    // task 2: listen for incoming tokens and verify tokenomics via dex_screener
    let handle2 = tokio::spawn(async move {
        /*         dex_mem.loop_awaiting_liquidity_tokens().await; */
        loop_yet_to_dexscreener().await;
    }); */

    let handle4 = tokio::spawn(async move {
        /* let mut asfasf = clone.lock().unwrap();
        asfasf.loop_awaiting_liquidity_tokens().await; */
        let dex_client = DexClient::new();
        run(tx_token_data, rx1, dex_client).await;
    });

    let handle_3 = tokio::spawn(async move {
        xyz_client.loop_token_reliability_info().await;
        //check for memory
    });

    let handle_purchase_token = tokio::spawn(async move {
        let jup_api = JupiterSwapApiClient::default();
        token_wtf(rx_token_data).await;
        /* //tokens are only received if they meet the requirements
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
        } */
    });

    tokio::join!(handle, handle_purchase_token, handle_3, handle4);
    /*         , handle, handle_3); */
    /*     handle4); */

    Ok(())
}

pub async fn token_wtf(mut rx_token_data: Receiver<TokenRiskMetaData>) {
    let jup_api = JupiterSwapApiClient::default();
    //tokens are only received if they meet the requirements
    let mut tokens: HashSet<Pubkey> = HashSet::new();
    // let mut temp_hashmap: HashMap<String, bool> = HashMap::new(); //if this is true
    while let Some(token_meta_data) = rx_token_data.recv().await {
        match token_meta_data {
            TokenRiskMetaData::DexScreenerResponse(dex) => {
                if tokens.get(&dex).is_some() {
                    // buy jup_api
                    if let Err(a) = jup_api.buy(dex).await {
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
}
