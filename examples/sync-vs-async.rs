use futures::future::join_all;
use std::time::{Duration, Instant};
use tokio::time::sleep;

#[tokio::main]
async fn main() {
    let before = Instant::now();
    sync_main(11).await;
    async_main(11).await;
    println!("Time elapsed: {:?}", before.elapsed());
}

async fn slow(id: i32, fn_type: &str) {
    println!("({}) Hello from slow_{}", fn_type, id);
    let time_sleep = Duration::from_secs(1);
    sleep(time_sleep).await;
    println!("({}) Bye from slow_{}", fn_type, id);
}

async fn sync_main(max: i32) {
    let before = Instant::now();
    for i in 1..max {
        slow(i, "sync").await;
    }
    println!("[Sync] Time elapsed: {:?}", before.elapsed());
}

async fn async_main(max: i32) {
    let before = Instant::now();
    let v: Vec<_> = (1..max).map(|i| slow(i, "async")).collect();
    let _ = join_all(v).await; // wait for all tasks to finish

    println!("[Async] Time elapsed: {:?}", before.elapsed());
}
