use crate::error::Result;
use model::{QuoteRequest, QuoteResponse};
use reqwest::Client;
use solana_sdk::pubkey;
use solana_sdk::pubkey::Pubkey;

const TEST_WALLET: Pubkey = pubkey!("2AQdpHJ2JpcEgPiATUXjQxA8QmafFegfQwSLWSprPicm");
const BASE_PATH: &str = "https://quote-api.jup.ag/v6";

mod field_as_string;
pub mod model;

#[derive(Clone)]
pub struct JupiterSwapApiClient {
    pub base_path: String,
}

impl Default for JupiterSwapApiClient {
    fn default() -> Self {
        Self {
            base_path: BASE_PATH.to_string(),
        }
    }
}

impl JupiterSwapApiClient {
    pub async fn quote(&self, quote_request: &QuoteRequest) -> Result<QuoteResponse> {
        let query = serde_qs::to_string(&quote_request)?;
        let response = Client::new()
            .get(format!("{}/quote?{query}", self.base_path))
            .send()
            .await?;
        // check_status_code_and_deserialize(response).await

        Ok(response.json::<QuoteResponse>().await?)
    }
}
