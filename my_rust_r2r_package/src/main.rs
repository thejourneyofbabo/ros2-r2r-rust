mod publisher;
mod subscriber;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().collect();

    if args.len() < 2 {
        println!("Usage: cargo run -- [publisher | subscriber]");
        return Ok(());
    }

    match args[1].as_str() {
        "publisher" => {
            publisher::run_publisher().await?;
        }
        "subscriber" => {
            subscriber::run_subscriber().await?;
        }
        _ => {
            println!("Invalid argument. Use either 'publisher' or 'subscriber'.");
        }
    }

    Ok(())
}
