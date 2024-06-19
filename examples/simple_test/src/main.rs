
use tokio::task::JoinHandle;
use tokio::time::{sleep, Duration};
use rand::Rng;
use rs_token::{Token, HttpTokenReceiver};

#[tokio::main]



async fn main() {
    let token = Token::<HttpTokenReceiver>::    builder()
        .url("localhost")
        .client("test-client")
        .password("test-client999")
        .build(HttpTokenReceiver::default()).await.unwrap();
    println!("Hello, world: token ...");

    let mut handles: Vec<JoinHandle<()>> = vec![];

    for i in 0..10 {
        let handle = tokio::spawn({
            let t = token.clone();
            let mut rng = rand::thread_rng();
            let sleep_duration = rng.gen_range(1..=5);
            async move {
                for iteration in 0..5 {    
                    {
                        let mut guard = t.lock().await;
                        let token_obj: &mut Token<HttpTokenReceiver> = &mut guard;
                        if let Ok(token_str) = token_obj.get().await {
                            println!("[{}] iteration: {}: token: {}", i, iteration, token_str);
        
                        } else {
                            println!("[{}] iteration: {}, error while requesting token", i, iteration);
                        }
                    }
                    println!("[{}] iteration: {}, Sleeping for {} seconds ...", i, iteration, sleep_duration);
                    sleep(Duration::from_secs(sleep_duration)).await;
                }
                println!("[{}] Done.", i);
            }
        });
        handles.push(handle);
    }

    for h in handles {
        let msg = format!("Task is done: {:?}", &h);
        let _ = h.await;
        println!("{}", msg);
    }
}
