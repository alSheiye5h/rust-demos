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


use tokio::sync::mpsc;

#[tokio::main]
async fn main() {
    let (tx, mut rx) = mpsc::channel(32); // Create a channel

    // Spawn a task that sends a message
    let task1 = tokio::spawn(async move {
        tx.send("Hello from task1").await.unwrap();
    });

    // Spawn a task that receives the message
    let task2 = tokio::spawn(async move {
        let message = rx.recv().await.unwrap();
        println!("Received: {}", message);
    });

    // Await tasks
    task1.await.unwrap();
    task2.await.unwrap();
}


use tokio::sync::{Mutex, mpsc};
use std::sync::Arc;

#[tokio::main]
async fn main() {
    let (tx, mut rx) = mpsc::channel(32); // Create a channel
    let data = Arc::new(Mutex::new(0)); // Shared data protected by a Mutex

    // Task 1: Modify shared state and send it via the channel
    let tx1 = tx.clone();
    let data1 = data.clone();
    tokio::spawn(async move {
        let mut data = data1.lock().await;
        *data += 1;
        tx1.send(*data).await.unwrap();
    });

    // Task 2: Modify shared state and send it via the channel
    let tx2 = tx.clone();
    let data2 = data.clone();
    tokio::spawn(async move {
        let mut data = data2.lock().await;
        *data += 2;
        tx2.send(*data).await.unwrap();
    });

    // Receive messages from the channel
    while let Some(val) = rx.recv().await {
        println!("Received: {}", val);
    }
}
