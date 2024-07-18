use std::sync::Arc;

use reqwest::{header, Client};
use tokio::{sync::mpsc::Sender, time::Instant};

use crate::models::TokenRiskMetaData;

use super::XyzTokenRisk;

#[derive(Debug)]
pub struct RugCheckClient {
    pub client: Arc<Client>,
    tx: Sender<(TokenRiskMetaData, bool)>,
}

impl RugCheckClient {
    pub fn new(tx: Sender<(TokenRiskMetaData, bool)>, client: Arc<Client>) -> Self {
        Self { client, tx }
    }
    /// Performs an HTTP `GET` request to the `https://api.dexscreener.com/latest/dex/search/?q=:q` path.
    pub async fn get_token_reliability_info(&self, addr: String) -> crate::error::Result<()> {
        let now = Instant::now();
        let path = format!("https://api.rugcheck.xyz/v1/tokens/{}/report/summary", addr);
        let r = self
            .client
            .get(path)
            .header(header::ACCEPT, "application/json")
            .send()
            .await
            .unwrap();

        let json = r.json::<XyzTokenRisk>().await?;

        self.tx.send(TokenRiskMetaData::XyzResponse(json)).await?;
        println!("xyz elasped: {:.3?}", now.elapsed());

        Ok(())
    }
}

#[tokio::test]
async fn test_get_token() {
    /* let client = RugCheckClient::default();

    println!(
        "{:?}",
        client
            .get_token_by_addr("HRsgxBZVeQ2qFoyyJDRbBswJpZxLqvGuAknVJprfpump".to_string())
            .await
    ); */
}
