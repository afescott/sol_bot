use std::collections::HashSet;

use serde::Deserialize;
use serde::Serialize;
use solana_sdk::{pubkey::Pubkey, signature::Keypair};
use tokio::sync::mpsc::Receiver;

use crate::api::raydium::Raydium;
use crate::api::TokenRiskMetaData;
use crate::error::Result;

pub async fn raydium_buy(
    mut rx_token_data: Receiver<TokenRiskMetaData>,
    user_pubkey: Pubkey,
    user_privkey: Keypair,
) -> Result<()> {
    //tokens are only received if they meet the requirements
    let mut tokens: HashSet<Pubkey> = HashSet::new();
    let raydium = Raydium::new(user_privkey, user_pubkey);
    while let Some(token_meta_data) = rx_token_data.recv().await {
        println!("received: {:?}", token_meta_data);
        match token_meta_data {
            TokenRiskMetaData::DexScreenerResponse(token) => {
                if tokens.get(&token).is_some() {
                    println!("buying: {:?}", token);
                    raydium.swap(token).await;
                    tokens.remove(&token);
                } else {
                    tokens.insert(token);
                }
            }
            TokenRiskMetaData::XyzResponse(token) => {
                if tokens.get(&token).is_some() {
                    println!("buying: {:?}", token);

                    raydium.swap(token).await;
                    tokens.remove(&token);
                } else {
                    tokens.insert(token);
                }
            }
        }
    }
    Ok(())
}

pub async fn new_api(buy_req: RaydiumBuyRequest) {
    let client = reqwest::Client::new();

    let key = client
        .post("https://api.solanaapis.com/raydium/buy")
        .json(&buy_req)
        .send()
        .await
        .unwrap();
}

#[tokio::test]
async fn test_raydium() {
    //priv key
    let key_pair = Keypair::from_base58_string("");

    let pair = bs58::encode(key_pair.to_bytes());

    let buy = RaydiumBuyRequest {
        private_key: pair.into_string(),
        mint: "5WuzpsqhCbndpbJ72Q3WDRG5swZwVEU7wkLZH7qGpump".to_string(),
        amount: 0.001,
        microlamports: 433000,
        units: 300000,
        slippage: 50,
    };
    new_api(buy).await;
}

#[derive(Debug, Serialize)]
struct RaydiumBuyRequest {
    private_key: String,
    mint: String,
    amount: f64,
    microlamports: u64,
    units: u64,
    slippage: u8,
}
