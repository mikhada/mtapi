# mtapi -- Motor Town API Interface
#### Copyright 2025, Aaron Johnson (Github User: Mikhada)

## Disclaimer: This is considered an "early access" project at the moment. If there are issues with updates, you may need to follow new installation instructions to reconcile as they may not be mentioned anywhere else.

**mtapi** is a command-line tool written in Rust for interacting with a [Motor Town: Behind The Wheel](https://store.steampowered.com/app/1369670/Motor_Town_Behind_The_Wheel/) dedicated game server's web API, enabling server administrators to query server status, send messages, ban/unban players, and more.

## Features
- Query server status and player lists
- Send in-game chat messages as announcements
- Kick, ban, and unban players
- Support for human-readable "pretty" or raw JSON output
- Config file support with optional CLI overrides

## Installation
```sh
git clone https://github.com/mikhada/mtapi.git
cd mtapi
cargo build --release
```

The compiled binary will be at `target/release/mtapi`.

## Configuration
By default, `mtapi` will look for a configuration file in one of two places:
- `$HOME/.config/mtapi.toml`
- `./mtapi.toml` (current directory)

Example config file (also found in `mtapi_example.toml`):
```toml
host = "http://localhost"
port = 8080
api_password = "changeit"
```

## Usage
```sh
mtapi [OPTIONS] <COMMAND> [ARGS...]
```

### Options
| Flag               | Description                           |
|--------------------|---------------------------------------|
| `-f, --config`     | Path to TOML config file              |
| `-s, --server`     | Override server hostname              |
| `-P, --port`       | Override server port                  |
| `-p, --password`   | Override API password                 |
| `--raw`            | Output raw JSON instead of pretty     |

### Commands
| Command     | Description                                    |
|-------------|------------------------------------------------|
| `version`   | Get server version                             |
| `count`     | Get number of players online                   |
| `players`   | List all connected players                     |
| `housing`   | List rented player houses                      |
| `banlist`   | Show the list of banned players                |
| `chat`      | Send chat message to the server (announcement) |
| `kick`      | Kick a player by `unique_id`                   |
| `ban`       | Ban a player (supports duration + reason)      |
| `unban`     | Unban a player by `unique_id`                  |

### Examples

**List players:**
```sh
mtapi players
```

**Send a chat message:**
```sh
mtapi chat "Hello, citizens."
```

**Kick a player:**
```sh
mtapi kick 1234567890abcdef
```

**Ban a player for 12 hours with a reason:**
```sh
mtapi ban 1234567890abcdef 12 "Griefing in the tuning shop"
```

**Unban a player:**
```sh
mtapi unban 1234567890abcdef
```

## Development
### Requires Rust v1.75+ (2024 Edition or later recommended)

The code is organized as follows:
- `main.rs` — command-line parsing, configuration loading, command dispatch
- `apiget.rs` — all HTTP GET endpoint wrappers
- `apipost.rs` — all HTTP POST endpoint wrappers
- `config.rs` — config file loading with support for CLI overrides

## License
Apache 2.0 License - See `LICENSE` for details, or visit [the Apache license page](http://www.apache.org/licenses/LICENSE-2.0).
