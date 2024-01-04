use anyhow::anyhow;
use regex::Regex;
use serenity::async_trait;
use serenity::model::channel::Message;
use serenity::model::gateway::Ready;
use serenity::prelude::*;
use serenity::Result;
use shuttle_secrets::SecretStore;
use tracing::info;

struct Bot;

#[async_trait]
impl EventHandler for Bot {
    async fn message(&self, _ctx: Context, msg: Message) {
        if msg.author.bot {
            return;
        }

        let Some((username, hash)) = match_url(&msg.content) else {
            return;
        };
        let reply = format!("https://vxtwitter.com/{}/status/{}\n", username, hash);
        check_msg(msg.reply(&_ctx.http, reply).await);
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

// 正規表現マッチングを行う関数
fn match_url(content: &str) -> Option<(String, String)> {
    let regex = Regex::new(
        r"https:\/\/(x|twitter)\.com\/(?<username>[a-zA-Z0-9_]{1,16})\/status\/(?<hash>[0-9]+)",
    )
    .unwrap();

    regex
        .captures(content)
        .map(|caps| (caps["username"].to_string(), caps["hash"].to_string()))
}

// テスト関数
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_match_url() {
        let content = "text https://twitter.com/user123/status/12345678 text";
        let (username, hash) = match_url(content).unwrap();
        assert_eq!(username, "user123");
        assert_eq!(hash, "12345678");
    }
}

#[shuttle_runtime::main]
async fn serenity(
    #[shuttle_secrets::Secrets] secret_store: SecretStore,
) -> shuttle_serenity::ShuttleSerenity {
    // Get the discord token set in `Secrets.toml`
    let token = if let Some(token) = secret_store.get("DISCORD_TOKEN") {
        token
    } else {
        return Err(anyhow!("'DISCORD_TOKEN' was not found").into());
    };

    // Set gateway intents, which decides what events the bot will be notified about
    let intents = GatewayIntents::GUILD_MESSAGES | GatewayIntents::MESSAGE_CONTENT;

    let client = Client::builder(&token, intents)
        .event_handler(Bot)
        .await
        .expect("Err creating client");

    Ok(client.into())
}
