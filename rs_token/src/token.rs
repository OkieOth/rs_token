use std::sync::Arc;
use tokio::sync::Mutex;
use time::OffsetDateTime;
use anyhow::{Result, anyhow};
use base64;

use crate::traits::TokenReceiver;

#[derive(Debug, Default)]
pub struct TokenContent {
    pub token: String,
    pub exiration: Option<OffsetDateTime>,
    pub last_updated: Option<OffsetDateTime>,
    pub last_checked: Option<OffsetDateTime>,
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

    pub fn validate(&mut self, token: &str) -> Result<bool> {
        let parts = token.split('.').collect::<Vec<_>>();
        if parts.len() != 3 {
            return Err(anyhow!("bad token content"));
        }
        // let header = base64::(parts[0])?;
        // let header: Header = serde_json::from_slice(&header)?;
    
        // // 2. Build the validation object
        // let mut validation = Validation::new(header.alg);
        // validation.set_issuer(Some(issuer.to_string()));
    
        // // 3. Retrieve the public key
        // let jwks_url = format!("{}/certs", issuer);
        // let client = reqwest::Client::new();
        // let response = client.get(Url::parse(&jwks_url)?)
        //     .send()?;
        // if !response.status().is_success() {
        //     return Err(jsonwebtoken::Error::HttpError(response.status()));
        // }
        // let jwks: serde_json::Value = response.json()?;
    
        // // 4. Extract the public key and verify the token
        // let key = jwks["keys"]
        //     .as_array()
        //     .ok_or(jsonwebtoken::Error::InvalidKeySet)?
        //     .iter()
        //     .find(|k| k["kid"] == header.kid)
        //     .ok_or(jsonwebtoken::Error::InvalidKeySet)?;
        // let pem = format!("-----BEGIN PUBLIC KEY-----\n{}\n-----END PUBLIC KEY-----\n", key["x5c"][0]);
        // let public_key = jsonwebtoken::DecodingKey::from_pem(&pem)?;
    
        // decode::<serde_json::Value>(token, &public_key, &validation)
        //     .map(|_| true)
        //     .map_err(|err| err.into())
        Ok(true)
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