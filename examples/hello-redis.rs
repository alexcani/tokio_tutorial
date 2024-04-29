use mini_redis::{client, Result};

#[tokio::main]
async fn main() -> Result<()> {
    // Open connection to mini-redis server
    let mut client = client::connect("127.0.0.1:6379").await?;

    // Set key
    client.set("hello", "world".into()).await?;
    client.set("nice", "view".into()).await?;

    // Get key
    let result = client.get("hello").await?;
    println!("Got value from the server. result = {:?}", result);

    let result = client.get("nice").await?;
    println!("Got value from the server. result = {:?}", result);

    Ok(())
}
