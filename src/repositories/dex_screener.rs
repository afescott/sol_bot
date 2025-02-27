use chrono::{DateTime, Utc};
use solana_sdk::pubkey::Pubkey;
use std::{collections::HashMap, time::Duration};
use tokio::sync::{broadcast::Receiver, mpsc::Sender};

use crate::api::{
    dexscreener::{api::DexClient, Liquidity, Timed, Transactions},
    Market, TokenRiskMetaData,
};

pub async fn run(
    tx: Sender<TokenRiskMetaData>,
    rx: Receiver<Market>,
    dex_client: DexClient,
) -> crate::error::Result<()> {
    let (tx_1, mut rx_1) = tokio::sync::mpsc::channel::<Pubkey>(50);
    /*     let dex_client = DexClient::new(client) */
    let producer_handle = tokio::spawn(loop_awaiting_liquidity_tokens(
        tx_1,
        tx.clone(),
        rx,
        dex_client.clone(),
    ));
    let consumer_handle = tokio::spawn(loop_yet_to_dexscreener(rx_1, dex_client, tx));

    tokio::join!(producer_handle, consumer_handle);
    // Wait for both tasks to finish

    Ok(())
}

pub async fn loop_awaiting_liquidity_tokens(
    tx_1: tokio::sync::mpsc::Sender<Pubkey>,
    tx: Sender<TokenRiskMetaData>,
    mut rx: Receiver<Market>,
    dex_client: DexClient,
) -> crate::error::Result<()> {
    println!("listening");
    while let Ok(result) = rx.recv().await {
        match dex_client
            .get_token_by_addr(result.token_address.to_string())
            .await
        {
            Ok(response) => {
                if let Some(re) = response.pairs {
                    if let Some(re) = re.first() {
                        if re.liquidity.clone().is_some_and(|x| x.usd > 2950.0)
                            && re.fdv.is_some_and(|x| x > 2950.0)
                        {
                            tx.send(TokenRiskMetaData::DexScreenerResponse(result.token_address))
                                .await?;
                        }
                    } else {
                        tx_1.send(result.token_address).await?;
                    }
                } else {
                    //not on dex_screener yet
                    tx_1.send(result.token_address).await?;
                }
            }

            Err(err) => println!("{:?}", err),
        };
    }
    Ok(())
}

pub async fn loop_yet_to_dexscreener(
    mut rx_1: tokio::sync::mpsc::Receiver<Pubkey>,
    dex_client: DexClient,
    tx: Sender<TokenRiskMetaData>,
) -> crate::error::Result<()> {
    println!("listening");
    let mut key_storage: Vec<(Pubkey, DateTime<Utc>)> = Vec::new();
    let mut transactions: HashMap<Pubkey, Timed<Transactions>> = HashMap::new();
    loop {
        if let Ok(res) = rx_1.try_recv() {
            let time = Utc::now();
            key_storage.push((res, time));
        }
        for ele in key_storage.clone() {
            match dex_client.get_token_by_addr(ele.0.to_string()).await {
                Ok(response) => {
                    if let Some(pairs) = response.pairs {
                        if let Some(re) = pairs.first() {
                            if re.txns.h6 > r.h6
                                && re.txns.h6.buys > 400
                                && re.txns.h6.buys > r.h6.sells
                                && re.txns.h6.buys > (re.txns.h6.sells * 2)
                                && re.liquidity.clone().is_some_and(|x| x.usd > 2950.0)
                                && re.fdv.is_some_and(|x| x > 2950.0)
                            {
                                println!("double");
                                key_storage.retain(|item| item.0.to_string() != ele.0.to_string());

                                tx.send(TokenRiskMetaData::DexScreenerResponse(ele.0))
                                    .await?;
                            } else {
                                key_storage.retain(|obj| {
                                    let time = Utc::now().signed_duration_since(obj.1);

                                    time.num_seconds() < 1000
                                });
                            }
                        }
                    }
                }

                Err(err) => println!("{:?}", err),
            };
        }
        tokio::time::sleep(Duration::from_secs(5)).await;
    }
}
