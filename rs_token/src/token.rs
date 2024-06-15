
#[derive(Debug)]
pub struct Token {
}

impl Token {
    pub fn builder() -> TokenBuilder {
        TokenBuilder::default()
    }
}

#[derive(Default)]
pub struct TokenBuilder {
}

impl TokenBuilder {
    pub fn dummy(&mut self, _s: &str) -> &mut Self {
        self
    }

    pub async fn build(&self) -> Token {
        Token {
        }
    }
}