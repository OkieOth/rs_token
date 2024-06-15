use std::sync::Arc;
use tokio::sync::Mutex;


use crate::traits::TokenReceiver;
use crate::token::TokenContent;
use anyhow::{Result, anyhow};

#[derive(Default)]
pub struct HttpTokenReceiver {
}

#[async_trait::async_trait]
impl TokenReceiver for HttpTokenReceiver {
    /// Do the full authentication and returns a token
    async fn get(url: &str, client: &str, password: &str, token_content: &mut Arc<Mutex<TokenContent>>) -> Result<()> {
        Err(anyhow!("TODO"))
    }

    /// Refreshs a token before expiration
    async fn refresh(url: &str, client: &str, password: &str, refresh_token: &str, token_content: &mut Arc<Mutex<TokenContent>>) -> Result<()> {
        Err(anyhow!("TODO"))
    }
}

