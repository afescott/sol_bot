use reqwest::Client;
use std::{sync::Arc, time::Duration};
use tokio::{sync::broadcast::Receiver, time::Instant};

use crate::api::{dexscreener::api::DexClient, Market};

#[derive(Debug)]
pub struct DexMem {
    rx: Receiver<Market>,
    dex_client: DexClient,
    storage: Vec<Market>,
}
impl DexMem {
    pub fn new(rx: Receiver<Market>, client: Arc<Client>) -> Self {
        Self {
            rx,
            dex_client: DexClient::new(client),
            storage: Vec::new(),
        }
    }

    pub async fn loop_awaiting_liquidity_tokens(mut self) {
        loop {
            let result = &self.rx.try_recv();
            if let Ok(result) = result {
                self.storage.push(*result);
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

                            self.storage
                                .retain(|item| item.token_address != ele.token_address);
                        } else {
                            println!("not on dex_screener yet");
                        }

                        println!("dex elapsed: {:.3?}", now.elapsed());
                    }

                    Err(err) => println!("{:?}", err),
                };

                tokio::time::sleep(Duration::from_secs(1)).await;
            }
        }
    }
}
