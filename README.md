# git-credential-ado-auth

A lightweight, minimalist Git credential helper for Azure DevOps (ADO) written in Rust. It acts as a bridge between Git and the Azure CLI, leveraging your existing `az login` sessions to seamlessly authenticate Git operations.

## Features

- **No Custom OAuth Flows**: Uses the Azure CLI's robust authentication state.
- **No Manual Token Management**: Relies on the Azure CLI for token caching and refreshing.
- **Zero Configuration**: Simply point Git to the binary and it works.
- **Lightweight**: Minimal code footprint, focusing on performance and security.

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

### 2. Configure Git

Point Git to the absolute path of the compiled binary:

```bash
# Update the path to your actual installation directory
git config --global credential.helper "/path/to/git-credential-ado-auth/target/release/git-credential-ado-auth"
```

*Note: Alternatively, you can move the binary to a directory in your `PATH` (e.g., `/usr/local/bin/`) and configure it with `git config --global credential.helper ado-auth`.*

## How it Works

When Git needs credentials for an Azure DevOps repository (e.g., `dev.azure.com` or `visualstudio.com`), it calls this helper. The helper uses the `azure_identity` crate to request an access token from the Azure CLI for the Azure DevOps resource ID (`499b84ac-1321-427f-aa17-267ca6975798`). It then returns this token to Git as the password, using `OAuth` as the username.

## Troubleshooting

If you encounter authentication errors, ensure you are logged into the Azure CLI:

```bash
az login
```

If Git cannot find the helper, ensure the path provided in `git config` is absolute and points to the correct binary.
