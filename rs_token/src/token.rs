use std::sync::Arc;
use tokio::sync::Mutex;
use time::PrimitiveDateTime;
use anyhow::{Result, anyhow};

use crate::traits::TokenReceiver;

#[derive(Debug, Default)]
pub struct TokenContent {
    pub token: String,
    pub exiration: Option<PrimitiveDateTime>,
    pub last_updated: Option<PrimitiveDateTime>,
    pub last_checked: Option<PrimitiveDateTime>,
}


#[derive(Debug)]
pub struct Token<T: TokenReceiver> {
    url: String,
    client: String,
    password: String,
    realm: String,

    refresh_duration: usize,
    content: Arc<Mutex<Option<TokenContent>>>,
    token_receiver: T,
}

impl<T: TokenReceiver> Token<T> {
    pub fn builder() -> TokenBuilder {
        TokenBuilder::default()
    }

    async fn init_if_needed(&mut self) -> Result<()> {
        let url_str = format!("{}/realms/{}/protocol/openid-connect/token", self.url, self.realm);
        self.token_receiver.get(&url_str, &self.client, &self.password, &mut self.content).await
    }

    pub async fn get(&mut self) -> Result<String> {
        self.init_if_needed().await?;
        let guard = self.content.lock().await;
        let content: &Option<TokenContent> = &guard;
        if let Some(tc) = content {
            Ok(tc.token.to_string())
        } else {
            Err(anyhow!("token not ready"))
        }
    }
}


#[derive(Default)]
pub struct TokenBuilder {
    realm: Option<String>,
    url: Option<String>,
    client: Option<String>,
    password: Option<String>,
    refresh_duration: Option<usize>,
}

impl TokenBuilder {
    pub fn url(&mut self, v: &str) -> &mut Self {
        self.url = Some(v.to_string());
        self
    }

    pub fn client(&mut self, v: &str) -> &mut Self {
        self.client = Some(v.to_string());
        self
    }

    pub fn password(&mut self, v: &str) -> &mut Self {
        self.password = Some(v.to_string());
        self
    }

    pub fn realm(&mut self, v: &str) -> &mut Self {
        self.realm = Some(v.to_string());
        self
    }

    pub fn refresh_duration(&mut self, v: usize) -> &mut Self {
        self.refresh_duration = Some(v);
        self
    }

    pub async fn build<T: TokenReceiver>(&self, receiver: T) -> Result<Arc<Mutex<Token<T>>>, String> {
        if self.url.is_none() {
            return Err("url isn't initialized".to_string());
        }
        if self.client.is_none() {
            return Err("client isn't initialized".to_string());
        }
        if self.password.is_none() {
            return Err("password isn't initialized".to_string());
        }
        if self.realm.is_none() {
            return Err("realm isn't initialized".to_string());
        }
        let refresh_duration = if let Some(rd) = self.refresh_duration {
            rd
        } else {
            30
        };
        let url = self.url.as_ref().unwrap();
        let client = self.client.as_ref().unwrap();
        let password = self.password.as_ref().unwrap();
        let realm = self.realm.as_ref().unwrap();
        Ok(Arc::new(Mutex::new(Token {
            url: url.clone(),
            realm: realm.clone(),
            client: client.clone(),
            password: password.clone(),
            refresh_duration,
            content: Arc::new(Mutex::new(None)),
            token_receiver: receiver,
        })))
    }
}