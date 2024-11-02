use crate::error::Result;
use jupiter_swap_api_client::quote::{QuoteRequest, QuoteResponse};
use jupiter_swap_api_client::swap::{SwapInstructionsResponseInternal, SwapRequest, SwapResponse};
use reqwest::Client;
use solana_sdk::pubkey;
use solana_sdk::pubkey::Pubkey;

const SOL_KEY: Pubkey = pubkey!("So11111111111111111111111111111111111111112");
const BASE_PATH: &str = "https://quote-api.jup.ag/v6";

#[derive(Clone)]
pub struct JupiterSwapApiClient {
    pub base_path: String,
    client: Client,
}

impl Default for JupiterSwapApiClient {
    fn default() -> Self {
        Self {
            client: Client::new(),
            base_path: BASE_PATH.to_string(),
        }
    }
}

impl JupiterSwapApiClient {
    pub async fn quote(&self, quote_request: &QuoteRequest) -> Result<QuoteResponse> {
        let response = self
            .client
            .get(format!(
                "https://quote-api.jup.ag/v6/quote?inputMint={:?}&outputMint={}&amount={}",
                quote_request.input_mint, quote_request.output_mint, quote_request.amount
            ))
            .header("Content-Type", "application/json")
            .send()
            .await?;

        Ok(response.json::<QuoteResponse>().await?)
    }

    pub async fn swap(&self, swap_request: &SwapRequest) -> Result<SwapResponse> {
        let response = self
            .client
            .post(format!("{}/swap", self.base_path))
            .json(swap_request)
            .header("Content-Type", "application/json")
            .send()
            .await?;

        Ok(response.json::<SwapResponse>().await?)
    }

    pub async fn swap_instructions(
        &self,
        swap_request: &SwapRequest,
    ) -> Result<SwapInstructionsResponseInternal> {
        let response = self
            .client
            .post(format!("{}/swap-instructions", self.base_path))
            .json(swap_request)
            .send()
            .await?;

        Ok(response.json::<SwapInstructionsResponseInternal>().await?)
    }
}

#[cfg(test)]
mod test {
    use std::time::Duration;

    use jupiter_swap_api_client::{
        quote::QuoteRequest,
        swap::SwapRequest,
        transaction_config::{DynamicSlippageSettings, TransactionConfig},
    };
    use solana_client::nonblocking::rpc_client::RpcClient;
    use solana_sdk::{
        commitment_config::CommitmentLevel, pubkey, pubkey::Pubkey, signature::Keypair,
        transaction::VersionedTransaction,
    };

    use crate::api::jupiter::JupiterSwapApiClient;
    const TEST_WALLET: Pubkey = pubkey!("qwCrxRSVHcos8aiQ4BjULJDyh2KqSdudUde4tdFQupv");

    pub async fn test_swap_jup(
        signed_versioned_transaction: VersionedTransaction,
    ) -> crate::error::Result<()> {
        let rpc_client = RpcClient::new("https://api.mainnet-beta.solana.com".into());

        let config = solana_client::rpc_config::RpcSendTransactionConfig {
            skip_preflight: true,
            preflight_commitment: Some(CommitmentLevel::Processed),
            max_retries: Some(5),
            ..Default::default()
        };

        let mut counter = 0;
        loop {
            println!("hit {:?}", counter);
            let res = rpc_client
                .send_transaction_with_config(&signed_versioned_transaction, config)
                .await;

            let res = rpc_client.get_signature_status(&res.unwrap()).await;

            let res = res.is_ok_and(|x| {
                x.is_some_and(|x| {
                    println!("{:?}", x);
                    x.is_ok()
                })
            });

            if res {
                println!("{:?}", res);
                return Ok(());
            }

            if counter > 15 {
                return Err(crate::error::ClientError::InvalidAddress(
                    "test".to_string(),
                ));
            }

            counter += 1;

            tokio::time::sleep(Duration::from_millis(500)).await;
        }
    }

    #[tokio::test]
    pub async fn asfafs() {
        let client = JupiterSwapApiClient::default();

        let quote_request = QuoteRequest {
            amount: 100,
            output_mint: NATIVE_MINT,
            input_mint: pubkey!("XjERkL1VGgef1yf9u6pvbtqdhbewrXYnjanJL91pump"),
            ..QuoteRequest::default()
        };
        let quote_response = client.quote(&quote_request).await.unwrap();

        let config = TransactionConfig {
            wrap_and_unwrap_sol: true,
            dynamic_compute_unit_limit: true,
            dynamic_slippage: Some(DynamicSlippageSettings {
                min_bps: None,
                max_bps: Some(500),
            }),
            ..Default::default()
        };
        let swap_req = &SwapRequest {
            user_public_key: TEST_WALLET,
            quote_response: quote_response.clone(),
            config,
        };

        let swap_response = client.swap(swap_req).await.unwrap();
        println!("Raw tx len: {}", swap_response.swap_transaction.len());

        let versioned_transaction: VersionedTransaction =
            bincode::deserialize(&swap_response.swap_transaction).unwrap();
        // Replace with a keypair or other struct implementing signer
        let key_pair = Keypair::from_base58_string("");

        let signed_versioned_transaction =
            VersionedTransaction::try_new(versioned_transaction.message, &[&key_pair]).unwrap();
        // send with rpc client...

        let rpc_client = RpcClient::new("https://api.mainnet-beta.solana.com".into());

        let config = solana_client::rpc_config::RpcSendTransactionConfig {
            skip_preflight: true,
            preflight_commitment: Some(CommitmentLevel::Processed),
            max_retries: Some(2),
            ..Default::default()
        };

        test_123(signed_versioned_transaction).await;
    }
}
