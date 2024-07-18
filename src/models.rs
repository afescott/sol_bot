use crate::api::{dexscreener::PairResponse, rugcheck::XyzTokenRisk};

pub enum TokenRiskMetaData {
    DexScreenerResponse(PairResponse),
    XyzResponse(XyzTokenRisk),
}
