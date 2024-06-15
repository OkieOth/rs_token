
use rs_token::Token;

#[tokio::main]

async fn main() {
    let token = Token::builder()
        .dummy("can be removed")
        .build().await;
    println!("Hello, world: token: {:?}", token);
}
