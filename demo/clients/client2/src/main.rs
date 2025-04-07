use reqwest::Client;
use tokio;
use std::env;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Get command line arguments
    let args: Vec<String> = env::args().collect();
    
    let default_port = "3001";
    // Default URL
    let mut url = String::from(format!("https://127.0.2.1:{}", default_port));
    
    // Parse arguments
    match args.len() {
        2 => {
            // Single argument case: "127.0.0.1:3000" or "127.0.0.1"
            let input = &args[1];
            if input.contains(':') {
                url = format!("https://{}", input);
            } else {
                url = format!("https://{}:{}", input, default_port);
            }
        }
        3 => {
            // Two argument case: "127.0.0.1" "3000"
            url = format!("https://{}:{}", args[1], args[2]);
        }
        _ => {
            println!("Usage: program [host:port] or [host] [port]");
            println!("Using default: {}", url);
        }
    }

    let client = Client::builder()
        .timeout(std::time::Duration::from_secs(15))
        .build()?;
    
    let response = client.get(&url).send().await;

    match response {
        Ok(res) => println!("Response: {:?}", res),
        Err(err) => {
            anyhow::bail!(err);
        },
    }

    Ok(())
}
