use crate::api::{Market, TokenRiskMetaData};

#[tokio::test]
pub async fn test() {
    let (tx, rx1) = tokio::sync::broadcast::channel::<Market>(50);
    let rx2 = tx.subscribe();

    let (tx_token_data, mut rx_token_data) = tokio::sync::mpsc::channel::<TokenRiskMetaData>(50);
}
