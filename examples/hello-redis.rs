use mini_redis::{client, Result};

#[tokio::main]
async fn main() -> Result<()> {
    let s1 = tokio::spawn(async {
        let mut client = client::connect("127.0.0.1:6379").await.unwrap();
        client.set("hello", "world".into()).await.unwrap();
        let result = client.get("hello").await.unwrap();
        println!("got value from the server; result={:?}", result);
    });
    let s2 = tokio::spawn(async {
        let mut client = client::connect("127.0.0.1:6379").await.unwrap();
        let result = client.get("hello").await.unwrap();
        println!("got value from the server; result={:?}", result);
    });

    s1.await.unwrap();
    s2.await.unwrap();

    Ok(())
}
