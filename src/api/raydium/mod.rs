use solana_client::nonblocking::rpc_client::RpcClient;
use solana_sdk::{pubkey::Pubkey, signature::Keypair, transaction::VersionedTransaction};
use std::sync::Arc;
use swap::{
    amm::executor::{RaydiumAmm, RaydiumAmmExecutorOpts},
    api_v3::ApiV3Client,
    types::{SwapExecutionMode, SwapInput},
};

use solana_sdk::pubkey;

const SOL: Pubkey = pubkey!("So11111111111111111111111111111111111111112");

pub struct Raydium {
    executor: RaydiumAmm,
    client: Arc<RpcClient>,
    key_pair: Keypair,
    user_pubkey: Pubkey,
}

impl Raydium {
    pub fn new(key_pair: Keypair, user_pubkey: Pubkey) -> Self {
        let client = Arc::new(RpcClient::new("https://api.mainnet-beta.solana.com".into()));
        let executor = RaydiumAmm::new(
            Arc::clone(&client),
            RaydiumAmmExecutorOpts::default(),
            ApiV3Client::new(None),
        );
        Self {
            executor,
            client,
            key_pair,
            user_pubkey,
        }
    }

    pub async fn swap(&self, token_pubkey: Pubkey) {
        let swap_input = SwapInput {
            input_token_mint: SOL,
            output_token_mint: token_pubkey,
            slippage_bps: 5000, // 50%
            amount: 10_000_000, // 1 SOL
            mode: SwapExecutionMode::ExactIn,
            market: None,
        };

        let quote = self.executor.quote(&swap_input).await;
        if let Ok(quote) = quote {
            let mut transaction = self
                .executor
                .swap_transaction(self.user_pubkey, quote, None)
                .await
                .unwrap();

            let blockhash = self.client.get_latest_blockhash().await.unwrap();
            transaction.message.set_recent_blockhash(blockhash);
            let _final_tx =
                VersionedTransaction::try_new(transaction.message, &[&self.key_pair]).unwrap();

            let signature = self.client.send_transaction(&_final_tx).await;
            println!("bought {:?}, signature:{:?}", token_pubkey, signature);
        } else {
        }
    }
}
