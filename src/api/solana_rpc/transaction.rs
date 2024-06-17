use std::str::FromStr;

use solana_sdk::pubkey::Pubkey;
use solana_transaction_status::{
    option_serializer::OptionSerializer, EncodedTransaction, EncodedTransactionWithStatusMeta,
    UiTransaction, UiTransactionStatusMeta,
};

use crate::api::solana_rpc::RaydiumMarket;

use super::Market;

const Base_Mint_Index: i32 = 7;
const Quote_Mint_Index: u8 = 8;

pub fn find_mint_token(transaction: EncodedTransactionWithStatusMeta) {
    let srm_pub_key = Pubkey::from_str("srmqPvymJeFKQ4zGQed1GFppgkRHL9kaELCbyksJtPX").unwrap();
    let mut transaction_conditions_met = true;

    if let EncodedTransaction::Json(transaction) = transaction.transaction {
        // println!("{:?}", transaction);
        let r = match transaction.message {
            solana_transaction_status::UiMessage::Parsed(parsed) => {
                println!("{:?}", parsed);
                let instructions = parsed.instructions;
                let key = &parsed.account_keys[0];

                // if instructions.len() < 10 {
                //     transaction_conditions_met = false;
                // } else {
                for ele in instructions {
                    match ele {
                        solana_transaction_status::UiInstruction::Compiled(compiled) => {
                            let instructions = compiled.accounts;

                            println!("{:?}", compiled.data);
                        }

                        solana_transaction_status::UiInstruction::Parsed(parsed) => match parsed {
                            solana_transaction_status::UiParsedInstruction::Parsed(parsed) => {
                                println!("parsed: {:?}", parsed);
                            }
                            solana_transaction_status::UiParsedInstruction::PartiallyDecoded(
                                decoded,
                            ) => println!("decoded: {:?}", decoded),
                        },
                    }
                }
            }
            solana_transaction_status::UiMessage::Raw(raw) => {
                println!("{:?}", raw);
                let instructions = raw.instructions;
                for ele in instructions {
                    if ele.accounts.len() < 10 {
                        transaction_conditions_met = false
                    } else {
                        // let result = safe_index(ele.accounts[8], raw.account_keys.clone());
                        let safe_index = |idx: u8| -> Pubkey {
                            println!(
                                "instruction number 8 {:?}, account instructions length: {:?}",
                                idx as usize,
                                raw.account_keys.len()
                            );
                            if idx as usize >= raw.account_keys.len() {
                                return srm_pub_key;
                            }
                            Pubkey::from_str(&raw.account_keys[idx as usize]).unwrap()
                        };

                        println!("{:?}", ele);
                        let mut market = super::Market {
                            market: safe_index(0),
                            event_queue: safe_index(2),
                            bids: safe_index(3),
                            asks: safe_index(4),
                            base_vault: safe_index(5),
                            quote_vault: safe_index(6),
                            base_mint: safe_index(7),
                            quote_mint: safe_index(8),
                        };

                        let vault_signer_nonce = &ele.data[23..31]; //?

                        let market_bytes = &market.market.to_bytes();

                        let data = vec![market_bytes, vault_signer_nonce.as_bytes()];

                        let vault_signer = Pubkey::create_program_address(
                            &data,
                            &Pubkey::from_str("675kPX9MHTjS2zt1qfr1NYHuzeLXfQM9H24wFSUt1Mp8")
                                .unwrap(),
                        )
                        .unwrap();

                        let ray = RaydiumMarket::new(market.market);

                        //next steps will take you away from the guide probs since you're using jupituer
                        // get the token requirements for jupiter transaction
                    }
                }
            }
        };
    }

    if transaction_conditions_met {}
}

pub fn create_raydium_transaction(market: Market) {}
