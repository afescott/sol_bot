use std::{sync::Arc, time::Duration};

use chrono::{DateTime, Utc};
use reqwest::{header, Client};
use solana_sdk::pubkey;
use solana_sdk::pubkey::Pubkey;
use tokio::{
    sync::{broadcast::Receiver, mpsc::Sender},
    time::Instant,
};

use crate::api::{Market, TokenRiskMetaData};

use super::XyzTokenRisk;

#[derive(Debug)]
pub struct RugCheckClient {
    pub client: Client,
    tx: Sender<TokenRiskMetaData>,
    storage: Vec<(Pubkey, DateTime<Utc>)>,
    rx: Receiver<Market>,
}

impl RugCheckClient {
    pub fn new(tx: Sender<TokenRiskMetaData>, rx: Receiver<Market>) -> Self {
        Self {
            client: Client::new(),
            tx,
            rx,
            storage: Vec::new(),
        }
    }
    /// Performs an HTTP `GET` request to the `https://api.dexscreener.com/latest/dex/search/?q=:q` path.
    pub async fn loop_token_reliability_info(&mut self) -> crate::error::Result<()> {
        loop {
            if let Ok(market) = self.rx.try_recv() {
                let time = Utc::now();

                self.storage.push((market.token_address, time));
            }
            for ele in self.storage.clone() {
                let path = format!(
                    "https://api.rugcheck.xyz/v1/tokens/{}/report/summary",
                    ele.0
                );
                match self
                    .client
                    .get(path)
                    .header(header::ACCEPT, "application/json")
                    .send()
                    .await
                {
                    Ok(r) => {
                        let token_risk = r.json::<XyzTokenRisk>().await;
                        if let Ok(token_risk) = token_risk {
                            if token_risk.score < 3000 {
                                self.tx.send(TokenRiskMetaData::XyzResponse(ele.0)).await?;

                                self.storage
                                    .retain(|item| item.0.to_string() != ele.0.to_string());
                            } else {
                                self.storage.retain(|obj| {
                                    let time = Utc::now().signed_duration_since(obj.1);

                                    time.num_seconds() < 300
                                });
                            }
                        }
                    }
                    Err(e) => println!("Error with rugcheck api"),
                }
            }

            tokio::time::sleep(Duration::from_secs(5)).await;
        }
    }
}

#[tokio::test]
async fn test_rugcheck_update() {
    let (tx, rx1) = tokio::sync::broadcast::channel::<Market>(50);

    let (tx_token_data, mut rx_token_data) = tokio::sync::mpsc::channel::<TokenRiskMetaData>(50);
    let mut client = RugCheckClient::new(tx_token_data, rx1);
    let handle_2 = tokio::spawn(async move {
        client.loop_token_reliability_info().await;
        //check for memory
    });

    let market = Market {
        token_address: pubkey!("H1oy6siM8ctrcpx4K7cxjrj51Hj2GdLKFvCCcprLpump"),
        market: pubkey!("MUYJorcyPGJqtRA8aRswfBRGRX6b7jHUDNPh7fYeQgY"),
        event_queue: pubkey!("DJhJ8TMyoHYucPjWnt7VzBosAdCGaX4xVGnbvstJ68yJ"),
        bids: pubkey!("HJYpfd6pxhxWPa2pvM9ABVrkmYesrcRSyvLvx7uYKtcA"),
        asks: pubkey!("iL5QRaLpRVBZhDF8grBFvfx67XeuZML9VwhRwGKGDpq"),
        base_vault: pubkey!("6L53YTFyekuMjFyPfnc6RGzrPL7dWntrnGiZZtN1AgSf"),
        quote_vault: pubkey!("9nuwv3xouccAVCnW2jX4E8XzyZhTXQUGqPDcwFycLv5q"),
        base_mint: pubkey!("B59L1j7VU4vMHMSxLJT5SFBZqdrzAcrjuDcjAHcuL76o"),
        quote_mint: pubkey!("BsBjjWZcqd1hRvfCjazafDwbgzDByNNQ3hKoJqHFvsKM"),
    };
    let handle_3 = tokio::spawn(async move {
        tx.send(market);
        //check for memory
    });

    tokio::join!(handle_3, handle_2);
}
