# Trakt CLI

A native command-line interface for the Trakt.tv API, written in Rust. Features a fully nested command structure mapping directly to the official Trakt.tv API documentation.

## Features

- **100% Trakt.tv API Coverage**: All endpoints, metadata, and user sync functionalities are fully supported.
- **Nested Command Structure**: Commands map directly to Trakt API blueprint routes.
- **OAuth2 Device Auth**: Secure login mechanism.
- **Native Cross-platform Support**: Binaries for Windows, macOS, and Linux (Intel and ARM).
- **JSON Output**: Easily pipe outputs into `jq` or other automation tools.

## Installation

Download the pre-compiled binaries for your architecture to have the tool permanently available from your PATH.

### macOS & Linux

```bash
# Get the latest release version
VERSION="v1.1.0"

# For macOS (Apple Silicon):
curl -sL "https://github.com/johnneerdael/trakt-cli/releases/download/${VERSION}/trakt-cli-darwin-arm64" -o trakt-cli

# For macOS (Intel):
# curl -sL "https://github.com/johnneerdael/trakt-cli/releases/download/${VERSION}/trakt-cli-darwin-amd64" -o trakt-cli

# For Linux (ARM64):
# curl -sL "https://github.com/johnneerdael/trakt-cli/releases/download/${VERSION}/trakt-cli-linux-arm64" -o trakt-cli

# For Linux (AMD64):
# curl -sL "https://github.com/johnneerdael/trakt-cli/releases/download/${VERSION}/trakt-cli-linux-amd64" -o trakt-cli

# Make executable and move to PATH
chmod +x trakt-cli
sudo mv trakt-cli /usr/local/bin/trakt-cli
```

### Windows

Open PowerShell as Administrator and run:

```powershell
$Version = "v1.1.0"
$Url = "https://github.com/johnneerdael/trakt-cli/releases/download/$Version/trakt-cli-windows-amd64.exe"
$Dest = "C:\Windows\System32\trakt-cli.exe"
Invoke-WebRequest -Uri $Url -OutFile $Dest
```

### From Source

```bash
cargo install --git https://github.com/johnneerdael/trakt-cli.git
# or
git clone https://github.com/johnneerdael/trakt-cli.git && cd trakt-cli && cargo install --path .
```

## Configuration

1. First, create an API app at https://trakt.tv/oauth/applications/new
2. Note your `client_id` and `client_secret`
3. Configure the CLI:

```bash
trakt-cli configure --client-id YOUR_CLIENT_ID --client-secret YOUR_CLIENT_SECRET
```

## Authentication

Run the device authentication flow:

```bash
trakt-cli auth
```

1. Request a device code from Trakt.
2. Display a verification URL and code.
3. Poll for authentication until you authorize the app.
4. Save your access token to the local config file automatically.

## Usage Overview

The CLI commands are heavily nested to categorize features logically. Check out `trakt-cli --help` for the full tree, or explore subcommands like `trakt-cli movies --help`.

### Finding Content
```bash
trakt-cli search text movie "Breaking Bad"
trakt-cli search text show "The Office"
```

### Browsing Movies & Shows
```bash
trakt-cli movies popular
trakt-cli movies trending
trakt-cli movies get "the-matrix-1999" --extended full
trakt-cli shows get "severance" --extended full
trakt-cli seasons get "severance" 1
trakt-cli episodes get "severance" 1 1
```

### Global Calendars
```bash
trakt-cli calendars movies
trakt-cli calendars shows
trakt-cli calendars premieres
```

### Sync & Library Management
Requires authentication (`trakt-cli auth`).
```bash
trakt-cli sync watchlist get
trakt-cli sync recommendations movies
trakt-cli checkin "the-matrix-1999"
```

### User Profiles & Social
```bash
trakt-cli users profile username
trakt-cli users lists get --id username --list-id 12345
trakt-cli users history username --type movies
```

## Options

- `--extended <EXTENDED>` - Extended info level (`none`, `images`, `full`, `full,images`, `metadata`)
- `--page <PAGE>` - Page number for paginated endpoints
- `--limit <LIMIT>` - Number of items per page limit
- `--json` - Format output as raw JSON string instead of pretty-printed console text
- `-q, --quiet` - Suppress logging output

## License

MIT License - see LICENSE file for details.
