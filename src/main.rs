use std::{collections::HashMap, sync::Arc};
use tokio::time::Instant;

use crate::{api::rugcheck::api::RugCheckClient, models::TokenRiskMetaData};
use api::Market;
use error::Result;
use repositories::dex_screener::DexMem;
pub use reqwest::{self, Client, IntoUrl, Url};

pub mod api;
mod error;
pub mod format;
mod models;
pub mod repositories;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let (tx, rx1) = tokio::sync::broadcast::channel::<Market>(100000);
    let (tx_token_data, mut rx_token_data) =
        tokio::sync::mpsc::channel::<(TokenRiskMetaData, bool)>(5000);

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

    let xyz_client = RugCheckClient::new(tx_token_data.clone(), client_2);
    let dex_mem = DexMem::new(tx_token_data, rx1, client_1);

    // task 2: listen for incoming tokens and verify tokenomics via dex_screener
    let handle_2 = tokio::spawn(async move {
        dex_mem.loop_awaiting_liquidity_tokens().await;
    });

    // task 3: use xyz to ensure token "legitimacy"
    let handle_3 = tokio::spawn(async move {
        //check for memory
        while let Ok(s) = rx2.recv().await {
            /*             if let Ok(s) = result { */
            println!("trying xyz");
            /*                 dex_client.loop_awaiting_liquidity_tokens(s).await; */
            let r = xyz_client
                .get_token_reliability_info(s.token_address.to_string())
                .await;
        }
    });

    let handle_purchase_token = tokio::spawn(async move {
        let mut legit_tokens: HashMap<bool, String> = HashMap::new();
        let mut temp_hashmap: HashMap<String, bool> = HashMap::new(); //if this is true
        while let Some(token_meta_data) = rx_token_data.recv().await {
            match token_meta_data {
                TokenRiskMetaData::DexScreenerResponse(dex) => {
                    let asfa = &dex.pairs.unwrap().first().unwrap().pair_address.clone();

                    if let Some(result) = temp_hashmap.get(asfa) {
                        if *result {
                            //proceed we can check this
                        } else {
                            temp_hashmap.remove(asfa);
                        }
                    }
                }
                TokenRiskMetaData::XyzResponse(xyz) => {
                    //this may be the wrong field
                    let token_program = &xyz.tokenProgram;

                    if let Some(result) = temp_hashmap.get(token_program) {
                        if *result {
                            //proceed we can check this
                        } else {
                            temp_hashmap.remove(token_program);
                        }
                    }
                }
            }
        }
    });

    tokio::join!(handle_2, handle, handle_3);
}
