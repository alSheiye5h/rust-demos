use tokio::sync::Mutex;
use std::sync::Arc;

async fn somefunc() {
    println!("Running somefunc");
}

#[tokio::main]
async fn main() {
    let mutex = Arc::new(Mutex::new(0));

    let mutex_clone = Arc::clone(&mutex);
    tokio::spawn(async move {
        let guard = mutex_clone.lock().await; // Task 1 locks
        somefunc().await; // Critical section - lock is held until `guard` is dropped
    });

    let mutex_clone = Arc::clone(&mutex);
    tokio::spawn(async move {
        let guard = mutex_clone.lock().await; // Task 2 waits for the lock
        println!("Got the lock!");
    });

    // This will run asynchronously
    tokio::time::sleep(std::time::Duration::from_secs(1)).await;
}
