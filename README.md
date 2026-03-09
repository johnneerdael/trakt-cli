# Trakt CLI

A native command-line interface for the Trakt.tv API, written in Rust.

## Features

- Full coverage of the Trakt.tv API
- OAuth2 Device Authentication for secure login
- Cross-platform support (Windows, macOS, Linux)
- JSON output for easy integration with other tools

## Installation

### From Source

```bash
cargo build --release
```

The binary will be located at `target/release/trakt-cli`.

### Pre-built Binaries

Download the latest release for your platform from the releases page.

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

This will:
1. Request a device code from Trakt
2. Display a verification URL and code
3. Poll for authentication until you authorize the app
4. Save your access token to the config file

## Usage

### Get User Settings

```bash
trakt-cli me
```

### Search

```bash
trakt-cli search --type movie "Breaking Bad"
trakt-cli search --type show "The Office"
```

### Browse Movies

```bash
trakt-cli movies-popular
trakt-cli movies-trending
trakt-cli movie "breaking-bad"
```

### Browse Shows

```bash
trakt-cli shows-popular
trakt-cli shows-trending
trakt-cli show "breaking-bad"
trakt-cli show-seasons "breaking-bad"
```

### Get Episode

```bash
trakt-cli episode "breaking-bad" 1 1
```

### User Data

```bash
trakt-cli user username
trakt-cli user-collection username --type movies
trakt-cli user-watchlist username --type shows
trakt-cli user-history username --type movies
trakt-cli user-ratings username
```

### Calendars

```bash
trakt-cli calendar-movies
trakt-cli calendar-shows
```

### Genres

```bash
trakt-cli genres-movies
trakt-cli genres-shows
```

### Lists

```bash
trakt-cli list 12345
trakt-cli list-items 12345
```

## Options

- `--extended <EXTENDED>` - Extended info level (none, images, full, full,images)
- `--page <PAGE>` - Page number
- `--limit <LIMIT>` - Number of items per page

## Development

### Requirements

- Rust 1.70+
- Cargo

### Building

```bash
cargo build --release
```

### Testing

```bash
cargo test
```

## License

MIT License - see LICENSE file for details.
