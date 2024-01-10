mod command;
mod db;

use anyhow::anyhow;
use anyhow::Context as _;
use command::match_set_command;
use command::match_url;
use serenity::async_trait;
use serenity::model::channel::Message;
use serenity::model::gateway::Ready;
use serenity::prelude::*;
use serenity::Result;
use shuttle_secrets::SecretStore;
use sqlx::Executor;
use sqlx::PgPool;
use tracing::info;

use crate::command::match_get_command;

struct Bot {
    database: PgPool,
}

#[async_trait]
impl EventHandler for Bot {
    async fn message(&self, _ctx: Context, msg: Message) {
        if msg.author.bot {
            return;
        }

        info!(msg.content);

        let guild_id = msg.guild_id.unwrap().to_string();
        let user_id = msg.author.id.to_string();

        if let Some((username, hash)) = match_url(&msg.content) {
            info!("regex match url!");
            let mode = db::get(&self.database, &guild_id, &user_id).await.unwrap();
            info!("user mode is {}", mode.clone());
            let domain = match mode.as_str() {
                "fixup" => "fixup",
                "fx" => "fxtwitter",
                _ => "vxtwitter",
            };
            let reply = format!("https://{}.com/{}/status/{}\n", domain, username, hash);
            info!(reply);
            check_msg(msg.reply(&_ctx.http, reply).await);
        }

        if let Some(mode) = match_set_command(&msg.content) {
            let result = db::set(&self.database, &mode, &guild_id, &user_id)
                .await
                .unwrap();
            let reply = format!("cmd: set, mode:{}\n", result);
            info!(reply);
            check_msg(msg.reply(&_ctx.http, reply).await);
        }
        if match_get_command(&msg.content) {
            let result = db::get(&self.database, &guild_id, &user_id)
                .await
                .unwrap_or_else(|_| "vx".to_string());
            let reply = format!("cmd: get, mode:{}\n", result);
            check_msg(msg.reply(&_ctx.http, reply).await);
        }
    }

    async fn ready(&self, _: Context, ready: Ready) {
        info!("{} is connected!", ready.user.name);
    }
}

fn check_msg(result: Result<Message>) {
    if let Err(why) = result {
        println!("Error sending message: {:?}", why);
    }
}

#[shuttle_runtime::main]
async fn serenity(
    #[shuttle_shared_db::Postgres] pool: PgPool,
    #[shuttle_secrets::Secrets] secret_store: SecretStore,
) -> shuttle_serenity::ShuttleSerenity {
    // Get the discord token set in `Secrets.toml`
    let token = if let Some(token) = secret_store.get("DISCORD_TOKEN") {
        token
    } else {
        return Err(anyhow!("'DISCORD_TOKEN' was not found").into());
    };

    pool.execute(include_str!("../schema.sql"))
        .await
        .context("failed to run migrations")?;

    let bot = Bot { database: pool };

    // Set gateway intents, which decides what events the bot will be notified about
    let intents = GatewayIntents::GUILD_MESSAGES | GatewayIntents::MESSAGE_CONTENT;

    let client = Client::builder(&token, intents)
        .event_handler(bot)
        .await
        .expect("Err creating client");

    Ok(client.into())
}
