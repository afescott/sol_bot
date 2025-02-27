use reqwest::{header, Client};
use url::Url;

use crate::{api::dexscreener::PairResponse, error::Result};

pub const BASE_URL: &str = "https://api.dexscreener.com/latest/";

/// A [Dexscreener API](https://docs.dexscreener.com/api/reference) HTTP client.
#[derive(Debug, Clone)]
pub struct DexClient {
    pub client: Client,
    pub url: Url,
}

impl DexClient {
    pub fn new() -> Self {
        Self {
            url: Url::parse(BASE_URL).unwrap(),
            client: reqwest::Client::new(),
        }
    }

    /// Performs an HTTP `GET` request to the `https://api.dexscreener.com/latest/dex/search/?q=:q` path.
    pub async fn get_token_by_addr(&self, addr: String) -> Result<PairResponse> {
        let path = self.url.join(format!("dex/tokens/{}", addr).as_str());
        let r = self
            .client
            .get(path.unwrap())
            .header(header::ACCEPT, "application/json")
            .send()
            .await?;

        Ok(r.json::<PairResponse>().await?)
    }
}

#[tokio::test]
async fn test_retain() {
    let mut asfa = vec![123, 213, 123];

    asfa.retain(|s| {
        {
            if *s == 123 {
                return true;
            }
        };
        false
    });
}
