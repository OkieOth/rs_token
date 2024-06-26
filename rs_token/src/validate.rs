use crate::traits::PublicKeyProvider;
use anyhow::{Result, anyhow};

pub struct TokenValidator<P: PublicKeyProvider> {
    public_key_provider: P,

}

impl TokenValidator {
    pub fn new(provider: P) -> Self {

    }
    pub async fn validate(&mut self, token: &str) -> Result(bool) {

    }
}

#[cfg(test)]
mod test {
    
}
