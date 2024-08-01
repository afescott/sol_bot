use std::sync::Arc;

use reqwest::{header, Client};
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
    storage: Vec<Pubkey>,
    rx: Receiver<Market>,
}

impl RugCheckClient {
    pub fn new(client: Client, tx: Sender<TokenRiskMetaData>, rx: Receiver<Market>) -> Self {
        Self {
            client,
            tx,
            rx,
            storage: Vec::new(),
        }
    }
    /// Performs an HTTP `GET` request to the `https://api.dexscreener.com/latest/dex/search/?q=:q` path.
    pub async fn loop_token_reliability_info(mut self) -> crate::error::Result<()> {
        loop {
            let now = Instant::now();

            let result = self.rx.try_recv();
            if let Ok(market) = result {
                self.storage.push(market.token_address);
            }
            for ele in self.storage.clone() {
                let path = format!("https://api.rugcheck.xyz/v1/tokens/{}/report/summary", ele);
                let r = self
                    .client
                    .get(path)
                    .header(header::ACCEPT, "application/json")
                    .send()
                    .await?;

                let token_risk = r.json::<XyzTokenRisk>().await?;

                println!("xyz elasped: {:.3?}", now.elapsed());

                println!("risk score, {:?}", token_risk.score);
                if token_risk.score < 3000 {
                    println!("risk score low, {:?}", token_risk.score);

                    self.tx.send(TokenRiskMetaData::XyzResponse(ele)).await?;

                    self.storage
                        .retain(|item| item.to_string() != ele.to_string());
                }
            }
        }
    }
}

#[tokio::test]
async fn test_rugcheck_update() {
    let client = RugCheckClient::new(Client::new());
    loop {
        println!(
            "{:?}",
            client
                .get_token_reliability_info(
                    "t8p4Scae2ntCetERHBSvJGrauSwBeAWUcMMRKZN6tjy".to_string()
                )
                .await
        );

        tokio::time::sleep(std::time::Duration::from_secs(4)).await;
    }
}
