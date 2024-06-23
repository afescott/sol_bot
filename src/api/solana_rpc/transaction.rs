use std::{
    collections::{HashMap, HashSet},
    str::FromStr,
};

use solana_sdk::pubkey::Pubkey;
use solana_transaction_status::{
    option_serializer::OptionSerializer, EncodedTransaction, EncodedTransactionWithStatusMeta,
    UiTransaction, UiTransactionStatusMeta,
};

use lazy_static::lazy_static;

use crate::api::solana_rpc::RaydiumMarket;

use super::Market;

const Base_Mint_Index: i32 = 7;
const Quote_Mint_Index: u8 = 8;

lazy_static! {
    static ref programs_to_avoid: HashSet<String> = {
        let mut set = HashSet::new();
        set.insert("srmqPvymJeFKQ4zGQed1GFppgkRHL9kaELCbyksJtPX".to_string());
        set.insert("So11111111111111111111111111111111111111112".to_string());
        set.insert("svarRent111111111111111111111111111111111".to_string());
        set.insert("11111111111111111111111111111111".to_string());
        set.insert("ComputeBudget111111111111111111111111111111".to_string());
        set.insert("SysvarRent111111111111111111111111111111111".to_string());
        set
    };
}

pub fn find_mint_token(transaction: EncodedTransactionWithStatusMeta) -> Option<Market> {
    let srm_pub_key = Pubkey::from_str("srmqPvymJeFKQ4zGQed1GFppgkRHL9kaELCbyksJtPX").unwrap();
    let mut transaction_conditions_met = true;

    if let EncodedTransaction::Json(transaction) = transaction.transaction {
        match transaction.message {
            solana_transaction_status::UiMessage::Parsed(parsed) => {
                println!("{:?}", parsed);
                let instructions = parsed.instructions;
                let key = &parsed.account_keys[0];

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
                let instructions = raw.clone().instructions;
                for ele in instructions {
                    let possible_srm_key = raw.account_keys.get(ele.program_id_index as usize);
                    if let Some(srm) = possible_srm_key {
                        if srm != "srmqPvymJeFKQ4zGQed1GFppgkRHL9kaELCbyksJtPX" {
                            transaction_conditions_met = false;
                        }
                    } else {
                        transaction_conditions_met = false;
                    }
                    if ele.accounts.len() < 10 || ele.data.len() < 37 {
                        transaction_conditions_met = false
                    }

                    if transaction_conditions_met {
                        println!("{:?}", raw);
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

                        let mut avoid = programs_to_avoid.get(&raw.account_keys[11]);

                        let potential_token_address = if avoid.is_some() {
                            println!(" not 11 ");
                            avoid = programs_to_avoid.get(&raw.account_keys[10]);
                            if avoid.is_some() {
                                println!(" not 10 ");
                                avoid = programs_to_avoid.get(&raw.account_keys[12]);
                                if avoid.is_some() {
                                    println!(" not 12 ");
                                    None
                                } else {
                                    Some(12)
                                }
                            } else {
                                Some(10)
                            }
                        } else {
                            Some(11)
                        };

                        if let Some(addr) = potential_token_address {
                            println!("token address: {:?}", potential_token_address);

                            let market = super::Market {
                                market: safe_index(0),
                                event_queue: safe_index(2),
                                bids: safe_index(3),
                                asks: safe_index(4),
                                base_vault: safe_index(5),
                                quote_vault: safe_index(6),
                                base_mint: safe_index(7),
                                quote_mint: safe_index(8),
                                token_address: safe_index(addr),
                            };

                            return Some(market);
                        }
                    }
                    /* let asfaf = spl_associated_token_account::get_associated_token_address(
                        &pub_key,
                        &token_address,
                    ); */

                    //next steps will take you away from the guide probs since you're using jupituer
                    // get the token requirements for jupiter transaction
                    /*                     } */

                    transaction_conditions_met = true;
                }
            }
        };
    }
    None

    /*     if transaction_conditions_met {} */
}

pub fn create_raydium_transaction(market: Market) {}

//excess
//
//
/*
let vault_signer_nonce = &ele.data[23..31]; //?
let market_bytes = &market.market.to_bytes();

let data = vec![market_bytes, vault_signer_nonce.as_bytes()];

//add more validation
 let vault_signer = Pubkey::create_program_address(
    &data,
    &Pubkey::from_str("srmqPvymJeFKQ4zGQed1GFppgkRHL9kaELCbyksJtPX")
        .unwrap(),
);

let ray = RaydiumMarket::new(market.market);

let token_address = market.base_mint;

let key = solana_sdk::bs58::decode(
    "qwCrxRSVHcos8aiQ4BjULJDyh2KqSdudUde4tdFQupv".as_bytes(),
)
.into_vec()
.unwrap();

let pub_key = Pubkey::new(&key); */
