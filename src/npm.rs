use std::collections::HashSet;
use std::fs;
use std::io::{self, Write};
use std::path::PathBuf;
use url::Url;

pub async fn handle_npm_setup<F, Fut>(get_token_fn: F)
where
    F: Fn() -> Fut,
    Fut: std::future::Future<Output = Result<String, Box<dyn std::error::Error>>>,
{
    let registries = find_azure_registries();
    if registries.is_empty() {
        println!("No Azure Artifacts registries found in .npmrc files.");
        return;
    }

    println!("Found {} Azure Artifacts registries.", registries.len());

    let token = match get_token_fn().await {
        Ok(t) => t,
        Err(e) => {
            eprintln!("Failed to get Azure DevOps token: {}", e);
            eprintln!("Ensure you are logged in with 'az login'.");
            std::process::exit(1);
        }
    };

    if let Err(e) = update_user_npmrc(&registries, &token) {
        eprintln!("Failed to update user .npmrc: {}", e);
        std::process::exit(1);
    }

    println!("Successfully updated ~/.npmrc with Azure Artifacts tokens.");
}

fn find_azure_registries() -> HashSet<String> {
    let mut registries = HashSet::new();
    let mut curr = std::env::current_dir().unwrap_or_else(|_| PathBuf::from("."));

    loop {
        let npmrc_path = curr.join(".npmrc");
        if npmrc_path.exists() {
            if let Ok(content) = fs::read_to_string(&npmrc_path) {
                for line in content.lines() {
                    let line = line.trim();
                    if line.is_empty() || line.starts_with('#') || line.starts_with(';') {
                        continue;
                    }

                    if let Some((key, value)) = line.split_once('=') {
                        let key = key.trim();
                        let value = value.trim();

                        if key == "registry" || key.ends_with(":registry") {
                            if is_azure_registry(value) {
                                if let Some(normalized) = normalize_registry_url(value) {
                                    registries.insert(normalized);
                                }
                            }
                        }
                    }
                }
            }
        }

        if let Some(parent) = curr.parent() {
            curr = parent.to_path_buf();
        } else {
            break;
        }
    }

    registries
}

fn is_azure_registry(url: &str) -> bool {
    url.contains("pkgs.dev.azure.com") || url.contains("visualstudio.com")
}

fn normalize_registry_url(url_str: &str) -> Option<String> {
    if let Ok(url) = Url::parse(url_str) {
        let host = url.host_str()?;
        let path = url.path();
        let mut normalized = format!("{}{}", host, path);
        if !normalized.ends_with('/') {
            normalized.push('/');
        }
        Some(normalized)
    } else {
        None
    }
}

fn update_user_npmrc(registries: &HashSet<String>, token: &str) -> io::Result<()> {
    let home_dir = home::home_dir().ok_or_else(|| io::Error::new(io::ErrorKind::NotFound, "Home directory not found"))?;
    let user_npmrc_path = home_dir.join(".npmrc");

    let mut new_lines = Vec::new();
    if user_npmrc_path.exists() {
        let content = fs::read_to_string(&user_npmrc_path)?;
        for line in content.lines() {
            let mut skip = false;
            for reg in registries {
                let prefix = format!("//{}", reg);
                if line.starts_with(&prefix) && line.contains(":_authToken=") {
                    skip = true;
                    break;
                }
            }
            if !skip {
                new_lines.push(line.to_string());
            }
        }
    }

    for reg in registries {
        new_lines.push(format!("//{}:_authToken={}", reg, token));
    }

    let mut file = fs::File::create(&user_npmrc_path)?;
    for line in new_lines {
        writeln!(file, "{}", line)?;
    }

    Ok(())
}
