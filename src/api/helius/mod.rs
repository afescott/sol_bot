use reqwest::Client;

const API_URL_V1: &str = "https://api-mainnet.helius-rpc.com/v1";
const API_URL_V0: &str = "https://api-mainnet.helius-rpc.com/v0";
const DEV_API_URL_V0: &str = "https://api-devnet.helius-rpc.com/v0";
const DAS_URL: &str = "https://mainnet.helius-rpc.com";

pub struct Helius {
    pub api_key: String,
    pub client: Client,
}

impl Helius {
    pub fn new(api_key: String, client: Client) -> Self {
        Self { api_key, client }
    }
}
