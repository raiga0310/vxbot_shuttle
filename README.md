# Discord Bot

This is a Discord bot built using Serenity and PostgreSQL. It has the following features:
## Commands

- `get` - Get the user's current mode (fx or vx)
- `set <mode>` - Set the user's mode to either fx or vx
- `help` - Display help text

## Modes

The bot supports two modes:

- `fx` - Links will unfurl to fxtwitter.com
- `vx` - Links will unfurl to vxtwitter.com

The default mode is `vx`.
## Database

The bot uses PostgreSQL to store user modes. It has a simple modes table with guild_id, user_id, and mode columns.
Configuration

The bot requires a DISCORD_TOKEN environment variable or secret containing a valid Discord bot token.
## Running
```
cargo run
```
This will run the migrations before starting the bot.
## Deployment

The bot is set up to deploy easily on Shuttle. Just push to Shuttle and it will run the migrations and start the bot automatically.

### Secrets

Before you deploy, you must set discord token in `Secrets.toml`:

```toml
DISCORD_TOKEN="You can get discord_token from your dashboard on discord developer portal"
```

### Deploy!!

You required install `cargo-shuttle`
```sh
$ cargo install cargo-shuttle
```

and deploy
```sh
$ cargo shuttle deploy
```

### Disable idling

The shuttle project will be idled if the project has no request for the recent 30 minutes (According to [site](https://docs.shuttle.rs/getting-started/idle-projects)).
So, you can avoid project idling to be disabled `timeout-minutes`.

```sh
# on your shuttle project already has deployed
$ cargo shuttle project restart --timeout-minutes 0
```

if you want to confirm `timeout-minutes`, do this:

```sh
$ cargo shuttle project status
```
