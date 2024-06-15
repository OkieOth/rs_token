use std::sync::Arc;
use tokio::sync::Mutex;
use anyhow::Result;

use crate::token::TokenContent;

#[async_trait::async_trait]
pub trait TokenReceiver {
    /// Do the full authentication and returns a token
    async fn get(url: &str, client: &str, password: &str, token_content: &mut Arc<Mutex<TokenContent>>) -> Result<()>;

    /// Refreshs a token before expiration
    async fn refresh(url: &str, client: &str, password: &str, refresh_token: &str, token_content: &mut Arc<Mutex<TokenContent>>) -> Result<()>;
}