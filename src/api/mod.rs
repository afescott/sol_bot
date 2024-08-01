pub mod dexscreener;
pub mod jupiter;
pub mod rugcheck;
mod solana_rpc;
pub use solana_rpc::{Market, SolanaRpc};
use solana_sdk::pubkey::Pubkey;

//Response to either rugcheck_xyz or dex_screener
pub enum TokenRiskMetaData {
    DexScreenerResponse(Pubkey),
    XyzResponse(Pubkey),
}
