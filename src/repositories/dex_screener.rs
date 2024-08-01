use reqwest::Client;
use solana_sdk::pubkey::Pubkey;
use std::{sync::Arc, time::Duration};
use tokio::{
    sync::{
        broadcast::Receiver,
        mpsc::{error::TryRecvError, Sender},
    },
    time::Instant,
};

use crate::api::{
    dexscreener::{api::DexClient, Pair},
    rugcheck::api::RugCheckClient,
    Market, TokenRiskMetaData,
};

#[derive(Debug)]
pub struct DexMem {
    tx: Sender<TokenRiskMetaData>,
    rx: Receiver<Market>,
    dex_client: DexClient,
    storage: Vec<Market>,
}
impl DexMem {
    pub fn new(tx: Sender<TokenRiskMetaData>, rx: Receiver<Market>, client: Client) -> Self {
        Self {
            tx,
            rx,
            dex_client: DexClient::new(client),
            storage: Vec::new(),
        }
    }

    pub async fn loop_awaiting_liquidity_tokens(mut self) -> crate::error::Result<()> {
        println!("listening");
        loop {
            let result = &self.rx.try_recv();

            if let Ok(result) = result {
                println!("received: {:?}", result);
                self.storage.push(*result);
            }
            for ele in self.storage.clone() {
                match self
                    .dex_client
                    .get_token_by_addr(ele.token_address.to_string())
                    .await
                {
                    Ok(response) => {
                        /*                         println!("count: {:?}", self.dex_client.dex.len()); */
                        if response.pairs.is_some() {
                            println!("dex_screener success: {:?}", ele.token_address.to_string());
                            let response = response.pairs;
                            if let Some(re) = response.expect("no pairs").first() {
                                //verify these
                                println!("fdv:: {:?}", re.fdv);
                                if re.fdv.expect("no fdv") > 10000.0 {}
                                self.tx
                                    .send(TokenRiskMetaData::DexScreenerResponse(ele.token_address))
                                    .await?;
                            }
                            self.storage
                                .retain(|item| item.token_address != ele.token_address);
                        } else {
                            //not on dex_screener yet
                        }
                        // println!("dex elapsed: {:.3?}", now.elapsed());
                    }

                    Err(err) => println!("{:?}", err),
                };
            }
        }
    }
}
