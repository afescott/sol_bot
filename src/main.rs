use clap::Parser;
use solana_sdk::{pubkey::Pubkey, signature::Keypair};
use std::str::FromStr;

use crate::{
    api::{dexscreener::api::DexClient, rugcheck::api::RugCheckClient},
    args::Args,
    repositories::{dex_screener::run, raydium::raydium_buy},
};
use api::{Market, TokenRiskMetaData};
use error::Result;
pub use reqwest::{self, Client, IntoUrl, Url};

pub mod api;
mod args;
mod error;
pub mod format;
pub mod repositories;

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();
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

    let mut xyz_client = RugCheckClient::new(tx_token_data.clone(), rx2);

    let handle4 = tokio::spawn(async move {
        let dex_client = DexClient::new();
        run(tx_token_data, rx1, dex_client).await;
    });

    let handle_3 = tokio::spawn(async move {
        xyz_client.loop_token_reliability_info().await;
        //check for memory
    });

    let handle_purchase_token = tokio::spawn(async move {
        let user_pubkey: Pubkey = Pubkey::from_str(&args.pub_key).unwrap();
        let user_privkey = Keypair::from_base58_string(&args.priv_key);
        raydium_buy(rx_token_data, user_pubkey, user_privkey).await;
    });

    tokio::join!(handle, handle_purchase_token, handle_3, handle4);

    Ok(())
}
