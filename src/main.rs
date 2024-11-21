use std::sync::Arc;

use anyhow::Context as _;
use serenity::model::channel::Message;
use serenity::model::gateway::Ready;
use serenity::prelude::*;
use serenity::{all::UserId, async_trait};
use shuttle_runtime::SecretStore;
use tracing::{error, info};

struct Bot {
    count: Arc<Mutex<u32>>,
}

// const GANGORRA_ID: UserId = UserId::new(183715899348353025);
const KAMIKAZE_ID: UserId = UserId::new(368133094831685642);

#[async_trait]
impl EventHandler for Bot {
    async fn message(&self, ctx: Context, msg: Message) {
        if msg.content.to_lowercase().contains("prega") {
            let mut locked_count = self.count.lock().await;
            *locked_count += 1;
            match msg.author.id {
                KAMIKAZE_ID => {
                    let response = format!("Kamikaze já perdeu {locked_count} pregas");

                    if let Err(why) = msg.channel_id.say(&ctx.http, response).await {
                        error!("Error sending message: {why:?}");
                    }

                    info!("Kamikaze falou prega {} vezes.", locked_count);
                }
                _ => (),
            }
        }
    }

    async fn ready(&self, _: Context, ready: Ready) {
        info!("{} is connected!", ready.user.name);
    }
}

#[shuttle_runtime::main]
async fn serenity(
    #[shuttle_runtime::Secrets] secrets: SecretStore,
) -> shuttle_serenity::ShuttleSerenity {
    // Get the discord token set in `Secrets.toml`
    let count = Arc::new(Mutex::new(1));
    let token = secrets
        .get("DISCORD_TOKEN")
        .context("'DISCORD_TOKEN' was not found")?;

    // Set gateway intents, which decides what events the bot will be notified about
    let intents = GatewayIntents::GUILD_MESSAGES | GatewayIntents::MESSAGE_CONTENT;

    let client = Client::builder(&token, intents)
        .event_handler(Bot {
            count: count.clone(),
        })
        .await
        .expect("Err creating client");

    Ok(client.into())
}
