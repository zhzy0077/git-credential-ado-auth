mod npm;

use azure_core::credentials::TokenCredential;
use azure_identity::AzureCliCredential;
use clap::{Parser, Subcommand};
use std::io::{self, BufRead};

#[derive(Parser)]
#[command(name = "git-credential-ado-auth", about = "Authentication helper for Azure DevOps")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Git credential helper: Retrieve credentials
    Get,
    /// Git credential helper: Store credentials (no-op)
    Store,
    /// Git credential helper: Erase credentials (no-op)
    Erase,
    /// Setup .npmrc for Azure Artifacts
    Npm,
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Get => handle_git_get().await,
        Commands::Store | Commands::Erase => {
            let _ = consume_stdin();
        }
        Commands::Npm => npm::handle_npm_setup(get_ado_token).await,
    }
}

async fn handle_git_get() {
    let _ = consume_stdin();
    match get_ado_token().await {
        Ok(token) => {
            println!("username=OAuth");
            println!("password={}", token);
        }
        Err(e) => {
            eprintln!("Failed to get Azure DevOps token via Azure CLI.");
            eprintln!("Error: {}", e);
            std::process::exit(1);
        }
    }
}

fn consume_stdin() -> Vec<String> {
    let stdin = io::stdin();
    let mut lines = Vec::new();
    for line in stdin.lock().lines() {
        let line = match line {
            Ok(l) => l,
            Err(_) => continue,
        };
        let trimmed = line.trim();
        if trimmed.is_empty() {
            break;
        }
        lines.push(trimmed.to_string());
    }
    lines
}

async fn get_ado_token() -> Result<String, Box<dyn std::error::Error>> {
    let credential = AzureCliCredential::new(None)?;
    let resource = "499b84ac-1321-427f-aa17-267ca6975798/.default";
    let token_response = credential.get_token(&[resource], None).await?;
    Ok(token_response.token.secret().to_string())
}
