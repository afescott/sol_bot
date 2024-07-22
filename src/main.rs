use solana_sdk::pubkey::{self, Pubkey};
use std::{
    collections::{HashMap, HashSet},
    sync::Arc,
};
use tokio::time::Instant;

use crate::api::rugcheck::api::RugCheckClient;
use api::{
    dexscreener::Pair,
    jupiter::{model::QuoteRequest, JupiterSwapApiClient},
    Market, TokenRiskMetaData,
};
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

    let (tx, rx1) = tokio::sync::mpsc::channel::<Market>(10000);
    let (tx_token_data, mut rx_token_data) = tokio::sync::mpsc::channel::<Pubkey>(10000);

    // let mut rx2 = tx.subscribe();

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
    let dex_mem = DexMem::new(tx_token_data, rx1, client_1, xyz_client);

    // task 2: listen for incoming tokens and verify tokenomics via dex_screener
    let handle_2 = tokio::spawn(async move {
        // while let Some(s) = rx1.recv().await {

        dex_mem.loop_awaiting_liquidity_tokens().await;
        // }
    });

    // task 3: use xyz to ensure token "legitimacy"
    // let handle_3 = tokio::spawn(async move {
    //     //check for memory
    //     while let Ok(s) = rx2.recv().await {
    //         let r = xyz_client
    //             .get_token_reliability_info(s.token_address.to_string())
    //             .await;
    //     }
    // });

    let jup_api = JupiterSwapApiClient::default();

    let handle_purchase_token = tokio::spawn(async move {
        //tokens are only received if they meet the requirements
        let mut purchased_tokens: HashSet<String> = HashSet::new();
        // let mut temp_hashmap: HashMap<String, bool> = HashMap::new(); //if this is true
        while let Some(token_meta_data) = rx_token_data.recv().await {
            let quote_request = QuoteRequest {
                amount: 1_000_000,
                input_mint: solana_sdk::pubkey!("So11111111111111111111111111111111111111112"),
                // this maybe wrong
                output_mint: token_meta_data,
                slippage_bps: 50,
                ..QuoteRequest::default()
            };

            println!("{quote_response:#?}");

            // POST /swap
            let swap_response = jupiter_swap_api_client
                .swap(&SwapRequest {
                    user_public_key: TEST_WALLET,
                    quote_response: quote_response.clone(),
                    config: TransactionConfig::default(),
                })
                .await
                .unwrap();

            // jup_api.quote()
        }
    });

    tokio::join!(handle_2, handle);

    Ok(())
}
