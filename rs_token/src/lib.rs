mod token;
mod traits;
mod http_receiver;
mod keycloak_pub_key;

pub use token::{Token, TokenBuilder};
pub use traits::{PublicKeyProvider, TokenReceiver};
pub use http_receiver::HttpTokenReceiver;
pub use keycloak_pub_key::KeycloakPubKeyProvider;

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
