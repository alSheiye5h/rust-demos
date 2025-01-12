use mini_redis::{client, Result};

#[tokio::main]
async fn main() -> Result<()> {
    // tconnecta b mini redis
    let mut client = client::connect("127.0.0.1:6379").await?;

    // set l value world l key hello
    client.set("hello", "world".into()).await?;

    // Get lkey hello
    let result = client.get("hello").await?;

    println!("got value from the server; result={:?}", result);

    Ok(())
}