use reqwest::Client;
use solana_sdk::pubkey::Pubkey;
use std::{sync::Arc, time::Duration};
use tokio::{
    sync::{mpsc::Receiver, mpsc::Sender},
    time::Instant,
};

use crate::api::{
    dexscreener::{api::DexClient, Pair},
    rugcheck::api::RugCheckClient,
    Market,
};

#[derive(Debug)]
pub struct DexMem {
    tx: Sender<Pubkey>,
    rx: Receiver<Market>,
    dex_client: DexClient,
    storage: Vec<Market>,
    rug_check_client: RugCheckClient,
}
impl DexMem {
    pub fn new(
        tx: Sender<Pubkey>,
        rx: Receiver<Market>,
        client: Arc<Client>,
        rug_check_client: RugCheckClient,
    ) -> Self {
        Self {
            tx,
            rx,
            dex_client: DexClient::new(client),
            storage: Vec::new(),
            rug_check_client,
        }
    }

    pub async fn loop_awaiting_liquidity_tokens(mut self) -> crate::error::Result<()> {
        println!("listening");
        loop {
            let result = &self.rx.try_recv();
            if let Ok(result) = result {
                let r = self
                    .rug_check_client
                    .get_token_reliability_info(result.token_address.to_string())
                    .await;

                if r.is_ok_and(|x| x) {
                    self.storage.push(*result);
                }
            }
            for ele in self.storage.clone() {
                let now = Instant::now();

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
                                if re.fdv.expect("no fdv") > 10000.0 {}
                                self.tx.send(ele.token_address).await?;
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

                tokio::time::sleep(Duration::from_secs(1)).await;
            }
        }
    }
}
