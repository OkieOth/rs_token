use std::sync::Arc;
use tokio::sync::Mutex;
use reqwest::Error;
use serde::{Deserialize, Serialize};
use time::{Duration, OffsetDateTime};


use crate::traits::TokenReceiver;
use crate::token::TokenContent;
use anyhow::{Result, anyhow};

#[derive(Default)]
pub struct HttpTokenReceiver {
}

#[derive(Serialize)]
struct TokenRequest {
    client_id: String,
    client_secret: String,
    grant_type: String,
}

#[derive(Deserialize, Debug)]
struct TokenResponse {
    access_token: String,
    token_type: String,
    expires_in: i64,
    refresh_token: String,
    scope: String,
}

#[async_trait::async_trait]
impl TokenReceiver for HttpTokenReceiver {
    /// Do the full authentication and returns a token
    async fn get(&mut self, url: &str, client: &str, password: &str, token_content: &mut Arc<Mutex<Option<TokenContent>>>) -> Result<()> {
        let http_client = reqwest::Client::new();

        let token_request = TokenRequest {
            client_id: client.to_string(),
            client_secret: password.to_string(),
            grant_type: "client_credentials".to_string(),
        };
        let response = http_client
        .post(url)
        .form(&token_request)
        .send()
        .await?;

        if response.status().is_success() {
            let token_response: TokenResponse = response.json().await?;
            let mut guard = token_content.lock().await;
            let expired_in = token_response.expires_in;
            let odt = OffsetDateTime::now_utc();
            let content: &mut Option<TokenContent> = &mut guard;
            *content = Some(TokenContent{
                token: token_response.access_token,
                exiration: odt.checked_add(Duration::seconds(expired_in)),
                last_checked: None,
                last_updated: None,
            });

            // println!("Access Token: {}", token_response.access_token);
            // println!("Token Type: {}", token_response.token_type);
            println!("Expires In: {}", token_response.expires_in);
            // println!("Refresh Token: {}", token_response.refresh_token);
            // println!("Scope: {}", token_response.scope);
            Ok(())
        } else {
            eprintln!("Error: {}", response.status());
            let error_text = response.text().await?;
            eprintln!("Error details: {}", error_text);
            Err(anyhow!(error_text))
        }
    }

    /// Refreshs a token before expiration
    async fn refresh(&mut self, url: &str, client: &str, password: &str, refresh_token: &str, token_content: &mut Arc<Mutex<TokenContent>>) -> Result<()> {
        Err(anyhow!("TODO"))
    }
}

