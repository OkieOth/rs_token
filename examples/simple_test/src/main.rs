
use tokio::task::JoinHandle;
use tokio::time::{sleep, Duration};
use rand::Rng;
use rs_token::Token;

#[tokio::main]



async fn main() {
    let token = Token::builder()
        .dummy("can be removed")
        .build().await;
    println!("Hello, world: token: {:?}", token);

    let mut handles: Vec<JoinHandle<()>> = vec![];

    for i in 0..10 {
        let handle = tokio::spawn({
            let t = token.clone();
            let mut rng = rand::thread_rng();
            let sleep_duration = rng.gen_range(1..=5);

            async move {
                let guard = t.lock().await;
                let token_obj: &Token = &guard;
                println!("[{}] Value in subtask: {:?}", i, token_obj);
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
