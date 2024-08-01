use std::{str::FromStr, time::Duration};

use crossbeam_channel::Receiver;
use solana_client::{
    rpc_client::RpcClient,
    rpc_config::RpcTransactionConfig,
    rpc_response::{Response, RpcLogsResponse},
};

use solana_pubsub_client::pubsub_client::PubsubClient;
use solana_sdk::{
    commitment_config::{CommitmentConfig, CommitmentLevel},
    pubkey::Pubkey,
    signature::Signature,
};
use solana_transaction_status::UiTransactionEncoding;
use std::sync::mpsc::Sender;

use crate::api::solana_rpc::transaction::find_mint_token;

use super::Market;

pub struct SolanaRpc {
    client: RpcClient,
    sender: tokio::sync::broadcast::Sender<Market>,
}

impl SolanaRpc {
    pub fn new(sender: tokio::sync::broadcast::Sender<Market>) -> Self {
        Self {
            client: RpcClient::new("https://api.mainnet-beta.solana.com"),
            sender,
        }
    }

    pub async fn get_transactions(&self) {
        let serum_openbook =
            solana_sdk::bs58::decode("srmqPvymJeFKQ4zGQed1GFppgkRHL9kaELCbyksJtPX".as_bytes())
                .into_vec()
                .unwrap();
        let (r, s) = PubsubClient::logs_subscribe(
            "wss://api.mainnet-beta.solana.com",
            solana_client::rpc_config::RpcTransactionLogsFilter::All,
            solana_client::rpc_config::RpcTransactionLogsConfig {
                commitment: Some(CommitmentConfig {
                    commitment: CommitmentLevel::Confirmed,
                }),
            },
        )
        .unwrap();

        &self.process_transaction(s).await;
    }

    async fn process_transaction(&self, r: Receiver<Response<RpcLogsResponse>>) {
        loop {
            let log_response = r.recv();

            if let Ok(log_response) = log_response {
                let signature = log_response.value.signature;
                let mut next = false;

                for ele in log_response.value.logs {
                    //follow guide and test at each if

                    if ele.contains("11111111111111111111111111111111") {
                        next = true;
                    }

                    // the transaction is ahead by one
                    if ele.contains("srmqPvymJeFKQ4zGQed1GFppgkRHL9kaELCbyksJtPX") {
                        if next {
                            let signature = solana_sdk::bs58::decode(signature.as_bytes())
                                .into_vec()
                                .unwrap();

                            let transcation_result = self.client.get_transaction_with_config(
                                &Signature::new(&signature),
                                RpcTransactionConfig {
                                    encoding: None,
                                    commitment: Some(CommitmentConfig::confirmed()),
                                    max_supported_transaction_version: Some(1),
                                },
                            );
                            if let Ok(transaction) = transcation_result {
                                let market = find_mint_token(transaction.transaction);

                                if let Some(market) = market {
                                    /*                                     println!("{:?}", market.token_address.to_string()); */
                                    &self.sender.send(market);
                                    tokio::time::sleep(Duration::from_secs(5)).await;
                                }
                            }
                        }
                        next = false;
                    }
                }
            }
        }
    }
}
