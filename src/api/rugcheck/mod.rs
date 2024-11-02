use serde::{Deserialize, Serialize};

pub mod api;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct XyzTokenRisk {
    pub score: i32,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Risk {
    name: String,
    value: String,
    description: String,
    score: u32,
    level: String,
}
