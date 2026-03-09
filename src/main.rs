use clap::Parser;
use anyhow::Result;

pub mod config;
pub mod auth;
pub mod client;
pub mod pagination;
pub mod api;
pub mod commands;
pub mod dispatch;

pub use commands::*;
pub use dispatch::*;

#[tokio::main]
async fn main() {
    let cli = Cli::parse();
    
    let exit_code = match run_cli(cli.clone()).await {
        Ok(code) => code,
        Err(e) => {
            if cli.json {
                let error_json = serde_json::json!({
                    "error": "failure",
                    "message": e.to_string()
                });
                eprintln!("{}", serde_json::to_string(&error_json).unwrap_or_default());
            } else {
                eprintln!("Error: {}", e);
            }
            EXIT_FAILURE
        }
    };
    
    std::process::exit(exit_code);
}
