use azure_core::credentials::TokenCredential;
use azure_identity::AzureCliCredential;
use clap::{Parser, Subcommand};
use std::io::{self, BufRead};

#[derive(Parser)]
#[command(name = "git-ado-auth", about = "Git credential helper for Azure DevOps using Azure CLI")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Retrieve credentials
    Get,
    /// Store credentials (no-op as Azure CLI manages caching)
    Store,
    /// Erase credentials (no-op as Azure CLI manages caching)
    Erase,
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    // Read git credential protocol input from stdin
    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        let line = match line {
            Ok(l) => l,
            Err(_) => continue,
        };
        let line = line.trim();
        if line.is_empty() {
            break; // Empty line signifies end of input in the git credential protocol
        }
    }

    match &cli.command {
        Commands::Get => {
            let credential = match AzureCliCredential::new(None) {
                Ok(c) => c,
                Err(e) => {
                    eprintln!("Failed to initialize Azure CLI credential.");
                    eprintln!("Error details: {}", e);
                    std::process::exit(1);
                }
            };
            
            // The well-known App ID for Azure DevOps
            let resource = "499b84ac-1321-427f-aa17-267ca6975798/.default";
            
            match credential.get_token(&[resource], None).await {
                Ok(token) => {
                    println!("username=OAuth");
                    println!("password={}", token.token.secret());
                }
                Err(e) => {
                    eprintln!("Failed to get Azure DevOps token via Azure CLI.");
                    eprintln!("Please ensure you have run 'az login' and are authenticated.");
                    eprintln!("Error details: {}", e);
                    std::process::exit(1);
                }
            }
        }
        Commands::Store => {
            // Safely ignore: Azure CLI manages token caching natively
        }
        Commands::Erase => {
            // Safely ignore: Azure CLI manages token caching natively
        }
    }
}
