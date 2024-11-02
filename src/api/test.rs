use anyhow::Result;
use solana_client::nonblocking::rpc_client::RpcClient;
use solana_sdk::{commitment_config::CommitmentConfig, pubkey};

use crate::api::{Market, TokenRiskMetaData};

#[tokio::test]
pub async fn test_balance() -> Result<()> {
    let rpc_client = RpcClient::new(
        "https://api.mainnet-beta.solana.com".into(),
        /*         CommitmentConfig::confirmed(), */
    );

    let msol = pubkey!("mSoLzYCxHdYgdzU16g5QSh3i5K3z3KZK7ytfqcJm7So");
    let pubkey = pubkey!("qwCrxRSVHcos8aiQ4BjULJDyh2KqSdudUde4tdFQupv");
    let msol_token_address = spl_associated_token_account::get_associated_token_address(
        &pubkey!("qwCrxRSVHcos8aiQ4BjULJDyh2KqSdudUde4tdFQupv"),
        &msol,
    );

    let r = rpc_client.get_token_account_balance(&pubkey);

    let msol_token_address =
        spl_associated_token_account::get_associated_token_address(&pubkey, &msol);
    println!(
        "Pre-swap SOL balance: {}",
        amount_to_ui_amount(rpc_client.get_balance(&pubkey).await.unwrap(), 9)
    );
    println!(
        "Pre-swap mSOL balance: {}",
        amount_to_ui_amount(
            rpc_client
                .get_token_account_balance(&msol_token_address)
                .await
                .unwrap()
                .amount
                .parse::<u64>()?,
            9
        )
    );

    Ok(())
}
#[tokio::test]
pub async fn test() {
    let (tx, rx1) = tokio::sync::broadcast::channel::<Market>(50);
    let rx2 = tx.subscribe();

    let (tx_token_data, mut rx_token_data) = tokio::sync::mpsc::channel::<TokenRiskMetaData>(50);
}

/// Convert a raw amount to its UI representation (using the decimals field
/// defined in its mint)
pub fn amount_to_ui_amount(amount: u64, decimals: u8) -> f64 {
    amount as f64 / 10_usize.pow(decimals as u32) as f64
}
