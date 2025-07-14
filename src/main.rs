mod config;
mod apiget;
mod apipost;

use config::Config;
use reqwest::Client;
use std::env;

pub struct ApiContext {
    pub base_url: String,
    pub password: String,
    pub client: Client,
}

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: mtapi <command> [args]");
        std::process::exit(1);
    }

    // Load configuration
    let config = Config::load().expect("Failed to load config.toml");
    let ctx = ApiContext {
        base_url: config.base_url(),
        password: config.api_password,
        client: Client::new(),
    };

    // Sanity check: ensure the API server is reachable
    let health_url = format!("{}/version?password={}", ctx.base_url, ctx.password);
    match ctx.client.get(&health_url).send().await {
        Ok(resp) => {
            if !resp.status().is_success() {
                eprintln!("API server responded with error: HTTP {}", resp.status());
                std::process::exit(11);
            }
        }
        Err(e) => {
            eprintln!("Could not connect to API server at {}:\n{}", ctx.base_url, e);
            std::process::exit(10);
        }
    }

    match args[1].as_str() {
        "playerlist" => {
            let json = apiget::player_list(&ctx).await.unwrap();
            println!("{}", json); // raw JSON, no pretty print
        }

        "playerban" => {
            if args.len() < 3 {
                eprintln!("Usage: mtapi playerban <unique_id>");
                std::process::exit(1);
            }
            let json = apipost::ban_player(&ctx, &args[2]).await.unwrap();
            println!("{}", json); // raw JSON
        }

        "version" => {
            let json = apiget::version(&ctx).await.unwrap();
            println!("{}", json);
        }

        _ => {
            eprintln!("Unknown command.");
            std::process::exit(1);
        }
    }
}

