use reqwest::{header, Client, RequestBuilder};
use serde::de::DeserializeOwned;
use url::Url;

use crate::error::Result;

pub const BASE_URL: &str = "https://api.dexscreener.com/latest/";

/// A [Dexscreener API](https://docs.dexscreener.com/api/reference) HTTP client.
#[derive(Clone, Debug)]
pub struct DexClient {
    pub client: Client,
    pub url: Url,
}

impl Default for DexClient {
    fn default() -> Self {
        Self {
            client: Client::new(),
            url: Url::parse(BASE_URL).unwrap(),
        }
    }
}

impl DexClient {
    fn _get(&self, path: &str) -> Result<RequestBuilder> {
        let url = self.url.join(path)?;
        Ok(self
            .client
            .get(url)
            .header(header::ACCEPT, "application/json"))
    }

    /// Performs an HTTP `GET` request to the `https://api.dexscreener.com/latest/dex/search/?q=:q` path.
    /* pub async fn search(&self, token: TokenType) -> Result<Pairs> {
        let token = match token {
            TokenType::Id(id) => {
                println!("id: {:?}", id);
                id
            }
            TokenType::Name(name) => {
                println!("{:?}", name);
                name
            }

            TokenType::Pairs(_pairs) => "blah".to_string(),
        };

        let path = self.url.join(format!("dex/search?q={}", token).as_str());

        let r = self
            .client
            .get(path.unwrap())
            .header(header::ACCEPT, "application/json")
            .send()
            .await?;

        Ok(r.json::<Pairs>().await?)
    } */

    async fn get_token<T: DeserializeOwned>(&self, path: &str) -> Result<String> {
        /*         Ok( */
        Ok(self
            ._get(path)?
            .send()
            .await?
            .error_for_status()?
            .json::<String>()
            .await
            .unwrap())
    }
}
