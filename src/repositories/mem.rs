use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

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

#[derive(Debug, Clone)]
pub struct StorageRepo<T>
where
    T: std::fmt::Debug + PartialEq,
    String: From<T>,
{
    // In memory state for each respective model type
    //
    // Key mapping to unique model types of a collection
    pub state: Arc<Mutex<HashMap<String, Vec<T>>>>,
}

impl<T> StorageRepo<T>
where
    String: From<T>,
    T: std::fmt::Debug + PartialEq,
{
    pub fn new() -> Self {
        Self {
            state: Arc::new(Mutex::new(HashMap::new())),
        }
    }
}

pub async fn find_token(pairs: Vec<Pairs>) {}

#[derive(Debug, Clone)]
pub enum RequestType {
    GetTokenInfo(u8),
}
