use tokio;
use std::sync::{Mutex, MutexGuard};

async fn do_something_async() {
    println!("h");
}

async fn increment_an_do_stuff(mutex: &Mutex<i32>) {
    { // 2 . wlkin mni drna inner scope li ghadropi mutex w ghtunlocka mni tsali hd scope khdm
        let mut mutex: MutexGuard<i32> = mutex.lock().unwrap();
        *mutex += 1;    
    }
    
    do_something_async().await;
}

#[tokio::main]
async fn main() {
    let mutex = Mutex::new(6);
    tokio::spawn(async move { // 1 . error hit std::sync::Mutex gaema m implementiya Send
        increment_an_do_stuff(&mutex).await;
    });
}
use tokio::task;

#[tokio::main]
async fn main() {
    // Spawn an async task
    let async_task = task::spawn(async {
        println!("Running async task...");
    });

    // Spawn a blocking task
    let blocking_task = task::spawn_blocking(|| {
        println!("Running blocking task...");
    });

    // Await both tasks
    async_task.await.unwrap();
    blocking_task.await.unwrap();
}
use tokio::task;

#[tokio::main]
async fn main() {
    let handle = task::spawn_blocking(|| {
        // Simulate blocking work
        println!("Running a blocking task!");
        42 // Return some result
    });

    let result = handle.await.unwrap();
    println!("Result: {}", result);
}
use deadpool_postgres::{Config, Manager, Pool};
use tokio_postgres::NoTls;

#[tokio::main]
async fn main() {
    // Create the pool configuration
    let mut cfg = Config::new();
    cfg.dbname = Some("example".to_string());
    cfg.user = Some("user".to_string());
    cfg.password = Some("password".to_string());

    // Build the pool
    let pool = cfg.create_pool(None, NoTls).unwrap();

    // Get a connection from the pool
    let client = pool.get().await.unwrap();

    // Use the connection
    let rows = client.query("SELECT * FROM users", &[]).await.unwrap();
    println!("Rows: {:?}", rows);
}
