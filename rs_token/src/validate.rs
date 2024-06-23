use crate::traits::PublicKeyProvider;
use anyhow::{Result, anyhow};

pub struct TokenValidator<P: PublicKeyProvider> {
    public_key_provider: P,

}

impl TokenValidator {
    pub async fn validate(&mut self, token: &str) -> Result(bool) {

    }
}

#[cfg(test)]
mod test {
    
}
