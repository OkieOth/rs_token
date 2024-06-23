use crate::traits::PublicKeyProvider;
use anyhow::{Error, Result, anyhow};
use serde_json::Value;

pub struct KeycloakPubKeyProvider {
    certs_url: String,
}

impl KeycloakPubKeyProvider {
    pub fn builder() -> KeycloakPubKeyProviderBuilder {
        KeycloakPubKeyProviderBuilder::default()
    }
}

#[derive(Default)]
pub struct KeycloakPubKeyProviderBuilder {
    server: Option<String>,
    port: Option<usize>,
    https: bool,
    realm: Option<String>,
}

impl KeycloakPubKeyProviderBuilder {
    pub fn server(&mut self, v: &str) -> &mut Self {
        self.server = Some(v.to_string());
        self
    }

    pub fn port(&mut self, v: usize) -> &mut Self {
        self.port = Some(v);
        self
    }

    pub fn https(&mut self, v: bool) -> &mut Self {
        self.https = v;
        self
    }

    pub fn realm(&mut self, v: &str) -> &mut Self {
        self.realm = Some(v.to_string());
        self
    }

    pub fn build(&self) -> Result<KeycloakPubKeyProvider> {
        let server = self.server.clone().ok_or_else(|| Error::msg("Server is missing"))?;
        let port = if let Some(p) = self.port {
            p.to_string()
        } else {
            "".to_string()
        };
        let realm = self.realm.clone().ok_or_else(|| Error::msg("Realm is missing"))?;
        let proto = if self.https {
            "https"
        } else {
            "http"
        };

        let url = format!("{}://{}{}/realms/{}/protocol/openid-connect/certs", proto, server, port, realm);
        Ok(KeycloakPubKeyProvider {
            certs_url: url,
        })
    }

}


#[async_trait::async_trait]
impl PublicKeyProvider for KeycloakPubKeyProvider {
    async fn get_key(&self) -> Result<Value> {
        let http_client = reqwest::Client::new();

        let response = http_client
        .get(&self.certs_url)
        .send()
        .await?;

        if response.status().is_success() {
            let j = response.json().await?;
            Ok(j)
        } else {
            eprintln!("Error: {}", response.status());
            let error_text = response.text().await?;
            eprintln!("Error details: {}", error_text);
            Err(anyhow!(error_text))
        }
    }

}