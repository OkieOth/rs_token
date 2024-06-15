use std::sync::Arc;
use tokio::sync::Mutex;
use time::PrimitiveDateTime;


#[derive(Debug, Default)]
pub struct TokenContent {
    pub token: String,
    pub refresh_tokeh: Option<String>,
    pub exiration: Option<PrimitiveDateTime>,
    pub last_updated: Option<PrimitiveDateTime>,
    pub last_checked: Option<PrimitiveDateTime>,
}


#[derive(Debug)]
pub struct Token<T> {
    url: String,
    client: String,
    password: String,

    refresh_duration: usize,
    content: Arc<Mutex<Option<TokenContent>>>,
    token_receiver: T,
}

impl<T> Token<T> {
    pub fn builder() -> TokenBuilder {
        TokenBuilder::default()
    }
}

#[derive(Default)]
pub struct TokenBuilder {
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

    pub fn refresh_duration(&mut self, v: usize) -> &mut Self {
        self.refresh_duration = Some(v);
        self
    }

    pub async fn build<T>(self, receiver: T) -> Result<Arc<Mutex<Token<T>>>, String> {
        if self.url.is_none() {
            return Err("url isn't initialized".to_string());
        }
        if self.client.is_none() {
            return Err("client isn't initialized".to_string());
        }
        if self.password.is_none() {
            return Err("password isn't initialized".to_string());
        }
        let refresh_duration = if let Some(rd) = self.refresh_duration {
            rd
        } else {
            30
        };
        let url = self.url.unwrap();
        let client = self.client.unwrap();
        let password = self.password.unwrap();
        Ok(Arc::new(Mutex::new(Token {
            url,
            client,
            password,
            refresh_duration,
            content: Arc::new(Mutex::new(None)),
            token_receiver: receiver,
        })))
    }
}