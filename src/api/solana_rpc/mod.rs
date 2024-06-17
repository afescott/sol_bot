use std::str::FromStr;

use solana_sdk::pubkey::Pubkey;

mod rpc;
mod transaction;

#[derive(Clone, Copy, Debug)]
pub struct Market {
    market: Pubkey,
    event_queue: Pubkey,
    bids: Pubkey,
    asks: Pubkey,
    base_vault: Pubkey,
    quote_vault: Pubkey,
    base_mint: Pubkey,
    quote_mint: Pubkey,
}

#[derive(Clone, Copy, Debug)]
pub struct RaydiumMarket {
    amm_id: Pubkey,
    amm_pool_coin_token_account: Pubkey,
    amm_pool_pc_token_account: Pubkey,
    amm_pool_token_mint: Pubkey,
    amm_target_orders: Pubkey,
    amm_open_orders: Pubkey,
}

impl RaydiumMarket {
    fn new<'a>(market: Pubkey) -> Self {
        let raydium_pub_key =
            &Pubkey::from_str("675kPX9MHTjS2zt1qfr1NYHuzeLXfQM9H24wFSUt1Mp8").unwrap();

        //fix this
        let calc_rayd_pubkey = |str: &'a str| {
            let r: Vec<Vec<u8>> = [
                raydium_pub_key.to_bytes().to_vec(),
                market.to_bytes().to_vec(),
                str.as_bytes().to_vec(),
            ]
            .into();
            let asdsad: &[&[u8]] = r
                .as_slice()
                .iter()
                .map(|inner_vec| inner_vec.as_ref())
                .collect();
        };
        // &[&[u8]]

        // Vec<Vec<u8>>
        Self {
            amm_id: Pubkey::find_program_address(
                calc_rayd_pubkey("amm_associated_seed"),
                raydium_pub_key,
            )
            .0,
            amm_pool_coin_token_account: Pubkey::find_program_address(
                calc_rayd_pubkey("coin_vault_associated_seed"),
                raydium_pub_key,
            )
            .0,
            amm_pool_pc_token_account: Pubkey::find_program_address(
                calc_rayd_pubkey("pc_vault_associated_seed"),
                raydium_pub_key,
            )
            .0,
            amm_pool_token_mint: Pubkey::find_program_address(
                calc_rayd_pubkey("lp_mint_associated_seed"),
                raydium_pub_key,
            )
            .0,
            amm_target_orders: Pubkey::find_program_address(
                calc_rayd_pubkey("target_associated_seed"),
                raydium_pub_key,
            )
            .0,
            amm_open_orders: Pubkey::find_program_address(
                calc_rayd_pubkey("open_order_associated_seed"),
                raydium_pub_key,
            )
            .0,
        }
    }
}
