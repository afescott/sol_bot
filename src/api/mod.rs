pub mod dexscreener;
pub mod jupiter;
pub mod rugcheck;
mod solana_rpc;
use dexscreener::Pair;
use rugcheck::XyzTokenRisk;
pub use solana_rpc::{Market, SolanaRpc};

//Response to either rugcheck_xyz or dex_screener
pub enum TokenRiskMetaData {
    DexScreenerResponse(Pair),
    XyzResponse(Option<XyzTokenRisk>),
}
