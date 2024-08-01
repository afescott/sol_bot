use serde::{Deserialize, Serialize};
use solana_sdk::pubkey::Pubkey;
use thiserror::Error;
use tokio::sync::mpsc::error::SendError;

use crate::api::TokenRiskMetaData;

/// Type alias for `Result<T, ClientError>`
pub type Result<T = (), E = ClientError> = std::result::Result<T, E>;

/// A Dexscreener error.
#[derive(Debug, Error)]
pub enum ClientError {
    #[error("Invalid address: {0}")]
    InvalidAddress(String),

    #[error(transparent)]
    Reqwest(#[from] reqwest::Error),

    #[error(transparent)]
    Url(#[from] url::ParseError),

    #[error(transparent)]
    Error(#[from] SendError<Pubkey>),

    #[error(transparent)]
    SerdeQsError(#[from] serde_qs::Error),

    #[error(transparent)]
    SendError(#[from] tokio::sync::mpsc::error::SendError<TokenRiskMetaData>),
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PairResponse<T> {
    // #[serde(rename = "schemaVersion")]
    // pub schema_version: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pairs: Option<Vec<T>>,
}
