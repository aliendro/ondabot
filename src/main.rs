use anyhow::Context as _;
use bot::handler::Handler;
use serenity::prelude::*;
use shuttle_runtime::SecretStore;

mod bot;
mod commands;

#[shuttle_runtime::main]
async fn serenity(
    #[shuttle_runtime::Secrets] secrets: SecretStore,
) -> shuttle_serenity::ShuttleSerenity {
    let token = secrets
        .get("DISCORD_TOKEN")
        .context("'DISCORD_TOKEN' was not found")?;

    // Set gateway intents, which decides what events the bot will be notified about
    let intents = GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::MESSAGE_CONTENT
        | GatewayIntents::GUILD_INTEGRATIONS;

    let client = Client::builder(&token, intents)
        .event_handler(Handler { secrets })
        .await
        .expect("Err creating client");

    Ok(client.into())
}
