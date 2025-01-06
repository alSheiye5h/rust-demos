// use mini_redis::{client, Result};

// #[tokio::main]
// async fn main() -> Result<()> {
//     // connect to mini-redis address
//     let mut client = client::connect("127.0.0.1:6379").await?;

//     // set the key "hello" with value world
//     client.set("hello", "world".into()).await?;

//     let result = client.get("hello").await?;

//     println!("got value from server, result={:?}", result.unwrap());

//     Ok(())
// }

