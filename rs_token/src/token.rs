use anyhow::{anyhow, Result};
use base64;
use std::sync::Arc;
use std::time::Instant;
use time::OffsetDateTime;
use tokio::sync::RwLock;
use tokio::time::{sleep, Duration};

use crate::traits::TokenReceiver;

#[derive(Debug, Default)]
pub struct TokenContent {
    pub token: String,
    pub exiration_seconds: i64,
    pub last_updated: Option<OffsetDateTime>,
    pub last_checked: Option<OffsetDateTime>,
}

pub type TokenContentArc = Arc<RwLock<Option<TokenContent>>>;
pub type TokenReceiverBox = Arc<RwLock<Box<dyn TokenReceiver + Send + Sync>>>;

pub struct Token {
    url: String,
    client: String,
    password: String,
    realm: String,

    refresh_duration: usize,
    content: TokenContentArc,
    token_receiver: TokenReceiverBox,
}


async fn get_expiration_seconds(
    last_updated: &Option<OffsetDateTime>,
    expiration_seconds: i64,
) -> Result<u64> {
    if let Some(lu) = last_updated {
        let odt = OffsetDateTime::now_utc();
        let sec_diff = odt.unix_timestamp() - lu.unix_timestamp();
        let secure_offset: i64 = 5;
        let remaining = expiration_seconds - secure_offset - sec_diff;
        if remaining > 0 {
            Ok(remaining as u64)
        } else {
            Ok(0)
        }
    } else {
        Err(anyhow!("no valid last updated"))
        }
}

async fn get_token_now(url_str: &str, client: &str, password: &str, content: TokenContentArc,token_receiver: TokenReceiverBox) -> Result<()> {
    let guard_receiver = token_receiver.read().await;
    let receiver: &dyn TokenReceiver = & **guard_receiver;
    receiver.get(url_str, client, password, content).await?;
    Ok(())
}


async fn renew_token(url_str: String, client: String, password: String, content: TokenContentArc, r: TokenReceiverBox) {
    fn get_error_duration(cur_sec_to_sleep: &mut u64) -> Duration {
        let max_sec_to_sleep: u64 = 60;
        let duration = Duration::from_secs(*cur_sec_to_sleep);
        if *cur_sec_to_sleep < max_sec_to_sleep {
            *cur_sec_to_sleep = *cur_sec_to_sleep * 2;
        }
        duration
    }
    let mut cur_sec_to_sleep = 1;
    loop {
        let d: Duration = {
            let guard = content.read().await;
            let cont: &Option<TokenContent> = &guard;
            if let Some(c) = cont {
                if let Ok(remaining_expiration) = get_expiration_seconds(&c.last_updated, c.exiration_seconds).await {
                    cur_sec_to_sleep = 1;
                    Duration::from_secs(remaining_expiration)
                } else {
                    get_error_duration(&mut cur_sec_to_sleep)
                }
            } else {
                get_error_duration(&mut cur_sec_to_sleep)
            }
        };
        sleep(d).await;
        let _ = get_token_now(&url_str, &client, &password, content.clone(), r.clone()).await;
    }
}

impl Token {
    pub fn builder() -> TokenBuilder {
        TokenBuilder::default()
    }

    async fn init_if_needed(&self) -> Result<()> {
        let url_str = format!(
            "{}/realms/{}/protocol/openid-connect/token",
            self.url, self.realm
        );
        let init_is_needed = {
            let guard = self.content.read().await;
            let content: &Option<TokenContent> = &guard;
            content.is_none()
        };
        if init_is_needed {
            {
                get_token_now(&url_str, &self.client, &self.password, self.content.clone(),self.token_receiver.clone()).await?;
            }
            tokio::spawn(renew_token(url_str.clone(), self.client.clone(), self.client.clone(), self.content.clone(),  self.token_receiver.clone()));
        }
        Ok(())
    }

    pub async fn get(&self) -> Result<String> {
        self.init_if_needed().await?;
        let guard = self.content.read().await;
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

    pub async fn build(
        &self,
        receiver: Box<dyn TokenReceiver + Send + Sync>,
    ) -> Result<Arc<RwLock<Token>>, String> {
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
        Ok(Arc::new(RwLock::new(Token {
            url: url.clone(),
            realm: realm.clone(),
            client: client.clone(),
            password: password.clone(),
            refresh_duration,
            content: Arc::new(RwLock::new(None)),
            token_receiver: Arc::new(RwLock::new(receiver)),
        })))
    }
}
