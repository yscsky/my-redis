use tokio::signal;

#[tokio::main]
async fn main() {
    match signal::ctrl_c().await {
        Ok(_) => println!("receive ctrl + c"),
        Err(err) => eprintln!("Unable to listen for shutdown signal: {}", err),
    }
    println!("Done");
}
