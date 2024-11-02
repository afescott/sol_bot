pub mod dexscreener;
pub mod jupiter;
pub mod raydium;
pub mod rugcheck;
mod solana_rpc;
mod test;
pub use solana_rpc::{Market, SolanaRpc};
use solana_sdk::pubkey::Pubkey;

//Response to either rugcheck_xyz or dex_screener
#[derive(Debug)]
pub enum TokenRiskMetaData {
    DexScreenerResponse(Pubkey),
    XyzResponse(Pubkey),
}
