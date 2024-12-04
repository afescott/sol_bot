use std::time::Duration;

use crate::api::jupiter::JupiterSwapApiClient;
use crate::error::Result;
use jupiter_swap_api_client::quote::QuoteRequest;
use jupiter_swap_api_client::swap::SwapRequest;
use jupiter_swap_api_client::transaction_config::{
    DynamicSlippageSettings, PrioritizationFeeLamports, TransactionConfig,
};
use solana_client::nonblocking::rpc_client::RpcClient;
use solana_sdk::commitment_config::CommitmentLevel;
use solana_sdk::pubkey;
use solana_sdk::pubkey::Pubkey;
use solana_sdk::signature::Keypair;
use solana_sdk::transaction::VersionedTransaction;

const USDC_MINT: Pubkey = pubkey!("EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v");
const NATIVE_MINT: Pubkey = pubkey!("So11111111111111111111111111111111111111112");

pub struct Jup {
    sol_client: RpcClient,
    user_pubkey: Pubkey,
    user_privkey: Keypair,
}

impl Jup {
    pub fn new(user_pubkey: Pubkey, user_privkey: Keypair) -> Self {
        Self {
            user_privkey,
            user_pubkey,
            /*             client: JupiterSwapApiClient::default(), */
            sol_client: RpcClient::new("https://api.mainnet-beta.solana.com".into()),
        }
    }

    pub async fn quote_swap(&self, output_mint: Pubkey) -> Result<()> {
        for _ in 0..4 {
            let signed_versioned_transaction = self.quote_sign(output_mint).await;

            println!("{:?}", signed_versioned_transaction);
            if let Ok(signed_version_transaction) = signed_versioned_transaction {
                self.swap(signed_version_transaction).await?;
                return Ok(());
            }
            println!("Failed once");

            tokio::time::sleep(Duration::from_millis(500)).await;
        }

        Ok(())
    }

    pub async fn quote_sign(&self, output_mint: Pubkey) -> Result<VersionedTransaction> {
        let quote_request = QuoteRequest {
            amount: 100,
            output_mint: output_mint,
            input_mint: USDC_MINT,
            ..QuoteRequest::default()
        };
        let client = JupiterSwapApiClient::default();
        let quote_response = client.quote(&quote_request).await?;

        /*         if let Ok(quote_response) = quote_response { */
        println!("quote-response");
        let config = TransactionConfig {
            wrap_and_unwrap_sol: true,
            dynamic_compute_unit_limit: true,
            dynamic_slippage: Some(DynamicSlippageSettings {
                min_bps: None,
                max_bps: None,
            }),
            ..Default::default()
        };
        let swap_req = &SwapRequest {
            user_public_key: self.user_pubkey,
            quote_response: quote_response.clone(),
            config,
        };

        let swap_response = client.swap(swap_req).await?;

        let versioned_transaction: VersionedTransaction =
            bincode::deserialize(&swap_response.swap_transaction).unwrap();

        let signed_versioned_transaction =
            VersionedTransaction::try_new(versioned_transaction.message, &[&self.user_privkey])
                .unwrap();

        Ok(signed_versioned_transaction)
    }

    pub async fn swap(
        &self,
        signed_versioned_transaction: VersionedTransaction,
    ) -> crate::error::Result<()> {
        let config = solana_client::rpc_config::RpcSendTransactionConfig {
            skip_preflight: true,
            preflight_commitment: Some(CommitmentLevel::Processed),
            max_retries: Some(5),
            ..Default::default()
        };

        let mut counter = 0;
        loop {
            println!("hit {:?}", counter);
            let res = self
                .sol_client
                .send_transaction_with_config(&signed_versioned_transaction, config)
                .await;

            let res = self.sol_client.get_signature_status(&res.unwrap()).await;

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

            if counter > 2 {
                return Err(crate::error::ClientError::SwapFail);
            }

            counter += 1;
        }
    }
}

#[tokio::test]
pub async fn tes123() {
    let test_key = pubkey!("qwCrxRSVHcos8aiQ4BjULJDyh2KqSdudUde4tdFQupv");
    let key_pair = Keypair::from_base58_string("");
    let jup = Jup::new(test_key, key_pair);

    let pubkey = pubkey!("F93dc9CPunC1R7GX7eHWQ3zKEgixoGruaxLVyV83pump");
    jup.quote_swap(pubkey).await;
}
