use chrono::{DateTime, Utc};
use reqwest::Client;
use solana_sdk::pubkey::Pubkey;
use std::time::Duration;
use tokio::sync::{broadcast::Receiver, mpsc::Sender};

use crate::api::{dexscreener::api::DexClient, Market, TokenRiskMetaData};

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
    /*         loop { */
    while let Ok(result) = rx.recv().await {
        match dex_client
            .get_token_by_addr(result.token_address.to_string())
            .await
        {
            Ok(response) => {
                if let Some(re) = response.pairs {
                    if let Some(re) = re.first() {
                        //verify these
                        println!("fdv:: {:?}", re.fdv);
                        if re.fdv.expect("no fdv") > 10000.0 {
                            println!("fdv over 10000");
                        }
                        tx.send(TokenRiskMetaData::DexScreenerResponse(result.token_address))
                            .await?;
                    } else {
                        tx_1.send(result.token_address).await?;
                    }
                } else {
                    tx_1.send(result.token_address).await?;
                    //not on dex_screener yet
                }
                // println!("dex elapsed: {:.3?}", now.elapsed());
            }

            Err(err) => println!("{:?}", err),
        };
    }
    Ok(())
    /*         } */
}

pub async fn loop_yet_to_dexscreener(
    mut rx_1: tokio::sync::mpsc::Receiver<Pubkey>,
    dex_client: DexClient,

    tx: Sender<TokenRiskMetaData>,
) -> crate::error::Result<()> {
    println!("listening");
    let mut key_storage: Vec<(Pubkey, DateTime<Utc>)> = Vec::new();
    loop {
        if let Ok(res) = rx_1.try_recv() {
            let time = Utc::now();
            key_storage.push((res, time));
        }
        for ele in key_storage.clone() {
            match dex_client.get_token_by_addr(ele.0.to_string()).await {
                Ok(response) => {
                    /*                         println!("count: {:?}", self.dex_client.dex.len()); */
                    if let Some(pairs) = response.pairs {
                        if let Some(re) = pairs.first() {
                            //verify these
                            println!("{:?}", re.liquidity);
                            if let Some(fdv) = re.fdv {
                                if fdv > 10000.0 {
                                    println!("fdv over 10000.0: {:?}", ele.0);
                                    key_storage
                                        .retain(|item| item.0.to_string() != ele.0.to_string());

                                    tx.send(TokenRiskMetaData::DexScreenerResponse(ele.0))
                                        .await?;
                                } else {
                                    key_storage.retain(|obj| {
                                        let time = Utc::now().signed_duration_since(obj.1);
                                        println!("since secs: {:?}", time.num_seconds());

                                        time.num_seconds() > 4500
                                    });
                                }

                                /* key_storage.retain(|obj| {
                                    println!("{:?}", obj.1);

                                    println!("sec blah: {:?}", (since_secs - obj.1));

                                    (since_secs - obj.1) > 25
                                }); */
                                /* key_storage
                                .retain(|item| item.0.to_string() != ele.0.to_string()); */
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
