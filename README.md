# git-ado-auth

A lightweight, minimalist authentication helper for Azure DevOps (ADO) written in Rust. It provides seamless authentication for both Git and npm/pnpm/yarn by leveraging your existing `az login` sessions.

## Features

- **Git Credential Helper**: Seamlessly authenticate Git operations using Azure CLI tokens.
- **npm Setup Tool**: Automatically detects Azure Artifacts registries and updates your `~/.npmrc` with fresh tokens.
- **No Custom OAuth Flows**: Uses the Azure CLI's robust authentication state.
- **No Manual Token Management**: Relies on the Azure CLI for token caching and refreshing.
- **Zero Configuration**: Simply run the tool and it works.

## Prerequisites

- [Azure CLI](https://learn.microsoft.com/en-us/cli/azure/install-azure-cli) installed and authenticated (`az login`).
- [Rust](https://www.rust-lang.org/tools/install) (for building from source).

## Installation

### 1. Build from Source

```bash
git clone https://github.com/zhzy0077/git-credential-ado-auth.git
cd git-credential-ado-auth
cargo build --release
```

### 2. Usage

#### Git Configuration

Point Git to the absolute path of the compiled binary:

```bash
# Update the path to your actual installation directory
git config --global credential.helper "/path/to/git-ado-auth/target/release/git-credential-ado-auth"
```

#### npm / Azure Artifacts Setup

Run the following command in your project directory (where your `.npmrc` is located):

```bash
/path/to/git-ado-auth/target/release/git-credential-ado-auth npm
```

The tool will:
1. Scan the current and parent directories for `.npmrc` files.
2. Identify any Azure Artifacts registries.
3. Acquire a token via the Azure CLI.
4. Update (or create) your user-level `~/.npmrc` with the necessary `_authToken` entries.

## How it Works

When the helper is called (either by Git or via the `npm` command), it uses the `azure_identity` crate to request an access token from the Azure CLI for the well-known Azure DevOps resource ID (`499b84ac-1321-427f-aa17-267ca6975798`).

- For **Git**, it returns this token as the password with `OAuth` as the username.
- For **npm**, it formats the token into `//<registry-url>:_authToken=<token>` and injects it into your global configuration, ensuring your project-level `.npmrc` remains clean of secrets.

## Troubleshooting

If you encounter authentication errors, ensure you are logged into the Azure CLI:

```bash
az login
```

If Git cannot find the helper, ensure the path provided in `git config` is absolute and points to the correct binary.
