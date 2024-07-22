use std::sync::Arc;

use reqwest::{header, Client};
use tokio::{sync::mpsc::Sender, time::Instant};

use crate::api::TokenRiskMetaData;

use super::XyzTokenRisk;

#[derive(Debug)]
pub struct RugCheckClient {
    pub client: Arc<Client>,
}

impl RugCheckClient {
    pub fn new(client: Arc<Client>) -> Self {
        Self { client }
    }
    /// Performs an HTTP `GET` request to the `https://api.dexscreener.com/latest/dex/search/?q=:q` path.
    pub async fn get_token_reliability_info(&self, addr: String) -> crate::error::Result<bool> {
        let now = Instant::now();

        let path = format!("https://api.rugcheck.xyz/v1/tokens/{}/report/summary", addr);
        let r = self
            .client
            .get(path)
            .header(header::ACCEPT, "application/json")
            .send()
            .await?;

        let token_risk = r.json::<XyzTokenRisk>().await?;

        println!("xyz elasped: {:.3?}", now.elapsed());
        if token_risk.score < 2000 {
            return Ok(true);
            // self.tx
            //     .send(TokenRiskMetaData::XyzResponse(Some(token_risk)))
            //     .await?;
        } else {
            return Ok(false);
            // self.tx.send(TokenRiskMetaData::XyzResponse(None)).await?;
        }
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
