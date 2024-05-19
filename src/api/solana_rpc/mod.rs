use crossbeam_channel::Receiver;
use solana_client::{
    rpc_client::RpcClient,
    rpc_response::{Response, RpcLogsResponse},
};

use solana_pubsub_client::pubsub_client::{LogsSubscription, PubsubClient};
use solana_sdk::{
    commitment_config::{CommitmentConfig, CommitmentLevel},
    pubkey::Pubkey,
    signature::Signature,
};
use solana_transaction_status::UiTransactionEncoding;

mod webhook;

pub const API_KEY: i32 = 26131853;
pub const API_HASH: &str = "645faae25d834346f3d4cd8751349fef";
const SESSION_FILE: &str = "dialogs.session";

struct Bot {
    client: RpcClient,
}

impl Bot {
    fn new() -> Self {
        Self {
            client: RpcClient::new("https://api.mainnet-beta.solana.com"),
        }
    }

    fn get_transactions(&self) {
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

        //this could also be a non-blocking client (which is actually more reliable for shutting down).
        //watch gjengset for next section on blocking or non-blocking
        /* let (r, s) = solana_pubsub_client::pubsub_client::PubsubClient::program_subscribe(
            "wss://api.mainnet-beta.solana.com",
            &Pubkey::new(&serum_openbook),
            None,
        )
        .unwrap(); */

        &self.process_transaction(s);
    }

    fn process_transaction(&self, r: Receiver<Response<RpcLogsResponse>>) {
        loop {
            let log_response = r.recv().unwrap();
            println!("{:?}", log_response);

            let r = log_response.value.signature;

            for ele in log_response.value.logs {
                //follow guide and test at each if
                if ele.contains("Program 11111111111111111111111111111111 success") {}
            }

            /*
            let rpc = solana_client::rpc_client::RpcClient::new("https://api.mainnet-beta.solana.com");
                 let asfa = asfa.unwrap().value.pubkey;
                let serum_openbook = solana_sdk::bs58::decode(asfa.as_bytes())
                    .into_vec()
                    .unwrap();

                let rasf = rpc
                    .get_signatures_for_address(&Pubkey::new(&serum_openbook))
                    .unwrap();

                for ele in rasf {
                    if ele.err.is_none() {
                        println!("{:?}", ele);

                        let signature = solana_sdk::bs58::decode(ele.signature.as_bytes())
                            .into_vec()
                            .unwrap();

                        let transaction = rpc
                            .get_transaction(&Signature::new(&signature), UiTransactionEncoding::Json);

                        if let Ok(transaction) = transaction {
                            if let solana_transaction_status::EncodedTransaction::Json(transaction) =
                                transaction.transaction.transaction
                            {
                                match transaction.message {
                                    solana_transaction_status::UiMessage::Parsed(parsed) => {
                                        for ele in parsed.instructions {
                                            match ele {
                                                solana_transaction_status::UiInstruction::Compiled(
                                                    compiled,
                                                ) => todo!(),
                                                solana_transaction_status::UiInstruction::Parsed(
                                                    parsed,
                                                ) => {
                                                    match parsed {
                                                        solana_transaction_status::UiParsedInstruction::Parsed(instruction) => todo!(),
                                                        solana_transaction_status::UiParsedInstruction::PartiallyDecoded(partial) => todo!(),
                                                    }
                                                    todo!()
                                                }
                                            }
                                        }
                                        todo!()
                                    }
                                    solana_transaction_status::UiMessage::Raw(raw) => {
                                        for ele in raw.instructions {}
                                        todo!()
                                    }
                                }
                            }
                        } else {
                            println!("error");
                        }
                    }
                } */
        }
    }
}

#[test]
fn channel() {
    let bot = Bot::new();

    bot.get_transactions();
}
