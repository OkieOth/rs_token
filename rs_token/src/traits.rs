use std::sync::Arc;
use tokio::sync::Mutex;
use serde_json::Value;
use anyhow::Result;

use crate::token::TokenContent;

#[async_trait::async_trait]
pub trait TokenReceiver {
    /// Do the full authentication and returns a token
    async fn get(&self, url: &str, client: &str, password: &str, token_content: Arc<Mutex<Option<TokenContent>>>) -> Result<()>;
}

#[async_trait::async_trait]
pub trait PublicKeyProvider {
    async fn get_key(&self) -> Result<Value>;
}