mod apiget;
mod apipost;
mod config;

use clap::Parser;
use config::Config;
use reqwest::Client;
use serde_json::Value;

#[derive(Parser, Debug)]
#[command(name = "mtapi", about = "Motor Town API command tool", version)]
struct Args {
    #[arg(short = 'f', long = "config", help = "Path to config file")]
    config_file: Option<String>,

    #[arg(short = 's', long = "server", help = "Override server hostname")]
    server: Option<String>,

    #[arg(short = 'P', long = "port", help = "Override server port")]
    port: Option<u16>,

    #[arg(short = 'p', long = "password", help = "Override API password")]
    password: Option<String>,

    #[arg(long = "raw", help = "Print raw JSON output instead of pretty")]
    raw: bool,

    #[arg(help = "Command to execute, like 'chat' or 'ban'")]
    command: String,

    #[arg(help = "Arguments for the command", trailing_var_arg = true)]
    command_args: Vec<String>,
}

pub struct ApiContext {
    pub base_url: String,
    pub password: String,
    pub client: Client,
}

fn print_output(json: Value, raw: bool) {
    if raw {
        println!("{}", json);
    } else {
        println!("{}", serde_json::to_string_pretty(&json).unwrap());
    }
}

#[tokio::main]
async fn main() {
    let args = Args::parse();

    // Load configuration from file
    let mut config = if let Some(path) = args.config_file.as_deref() {
        Config::load(Some(path)).expect("Failed to load config")
    } else {
        Config::load(None).expect("Failed to load config")
    };

    // Configuration overrides via flags
    if let Some(server) = args.server {
        config.host = server;
    }
    if let Some(port) = args.port {
        config.port = port;
    }
    if let Some(password) = args.password {
        config.api_password = password;
    }

    let ctx = ApiContext {
        base_url: config.base_url(),
        password: config.api_password,
        client: Client::new(),
    };

    // Sanity check
    let health_url = format!("{}/version?password={}", ctx.base_url, ctx.password);
    match ctx.client.get(&health_url).send().await {
        Ok(resp) => {
            if !resp.status().is_success() {
                eprintln!("API server responded with error: HTTP {}", resp.status());
                std::process::exit(11);
            }
        }
        Err(e) => {
            eprintln!(
                "Could not connect to API server at {}:\n{}",
                ctx.base_url, e
            );
            std::process::exit(10);
        }
    }

    match args.command.as_str() {
        // GET calls
        "count" => {
            let json = apiget::player_count(&ctx).await.unwrap();
            print_output(json, args.raw);
        }
        "players" | "list" | "plist" => {
            let json = apiget::player_list(&ctx).await.unwrap();
            print_output(json, args.raw);
        }
        "housing" | "houses" | "hlist" => {
            let json = apiget::housing_list(&ctx).await.unwrap();
            print_output(json, args.raw);
        }
        "banlist" | "blist" => {
            let json = apiget::player_banlist(&ctx).await.unwrap();
            print_output(json, args.raw);
        }
        "version" => {
            let json = apiget::version(&ctx).await.unwrap();
            print_output(json, args.raw);
        }
        // POST calls
        "chat" | "message" | "msg" | "send" => {
            if args.command_args.len() < 1 {
                eprintln!("Usage: mtapi chat '<message>'");
                std::process::exit(12);
            }
            let json = apipost::chat(&ctx, &args.command_args[0]).await.unwrap();
            print_output(json, args.raw);
        }
        "kick" => {
            if args.command_args.len() < 1 {
                eprintln!("Usage: mtapi kick <unique_id>");
                std::process::exit(12);
            }
            let json = apipost::player_kick(&ctx, &args.command_args[0])
                .await
                .unwrap();
            print_output(json, args.raw);
        }
        "ban" => {
            if args.command_args.len() < 1 {
                eprintln!("Usage: mtapi ban <unique_id> [hours] [reason]");
                std::process::exit(12);
            }
            let unique_id = &args.command_args[0];
            let hours = args.command_args.get(1).and_then(|h| h.parse::<u32>().ok());
            let reason = args
                .command_args
                .get(2)
                .map(|_r| args.command_args[2..].join(" "));
            let json = apipost::player_ban(&ctx, unique_id, hours, reason.as_deref())
                .await
                .unwrap();
            print_output(json, args.raw);
        }
        "deban" | "unban" => {
            if args.command_args.len() < 1 {
                eprintln!("Usage: mtapi unban <unique_id>");
                std::process::exit(12);
            }
            let json = apipost::player_unban(&ctx, &args.command_args[0])
                .await
                .unwrap();
            print_output(json, args.raw);
        }
        _ => {
            eprintln!("Unknown command.");
            std::process::exit(1);
        }
    }
}
