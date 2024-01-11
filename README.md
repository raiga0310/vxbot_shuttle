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
