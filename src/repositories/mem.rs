use serde::de::DeserializeOwned;

use super::models::Pairs;

#[async_trait::async_trait]
pub trait Provider: Send + Sync + 'static {
    fn new() -> Self;

    async fn get_user_info<T: DeserializeOwned + core::fmt::Debug, U: From<T>>(
        &self,
        request: RequestType,
    ) -> Result<Vec<U>, crate::error::ClientError>;
}

#[async_trait::async_trait]
pub trait StorageRepository<T> {
    fn store_data(&self, results: Vec<T>); //internal event state just needs selection, event
}

pub async fn find_token(pairs: Vec<Pairs>) {}

#[derive(Debug, Clone)]
pub enum RequestType {
    GetTokenInfo(u8),
}
