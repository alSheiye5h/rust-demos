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