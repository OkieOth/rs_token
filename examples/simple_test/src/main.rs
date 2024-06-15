
use tokio::task::JoinHandle;
use tokio::time::{sleep, Duration};
use rand::Rng;
use rs_token::{Token, HttpTokenReceiver};

#[tokio::main]



async fn main() {
    let token = Token::<HttpTokenReceiver>::builder()
        .build(HttpTokenReceiver::default()).await.unwrap();
    println!("Hello, world: token ...");

    let mut handles: Vec<JoinHandle<()>> = vec![];

    for i in 0..10 {
        let handle = tokio::spawn({
            let t = token.clone();
            let mut rng = rand::thread_rng();
            let sleep_duration = rng.gen_range(1..=5);

            async move {
                let guard = t.lock().await;
                let token_obj: &Token<HttpTokenReceiver> = &guard;
                println!("[{}] Value in subtask ... not implemented", i);
                println!("[{}] Sleeping for {} seconds ...", i, sleep_duration);
                sleep(Duration::from_secs(sleep_duration)).await;
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
