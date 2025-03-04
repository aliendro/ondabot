use anyhow::Context as _;
use bot::handler::Handler;
use chatgpt::prelude::*;
use serenity::prelude::*;
use shuttle_runtime::SecretStore;
use std::sync::Arc;

mod bot;
mod commands;

#[shuttle_runtime::main]
async fn serenity(
    #[shuttle_runtime::Secrets] secrets: SecretStore,
) -> shuttle_serenity::ShuttleSerenity {
    let discord_token = secrets
        .get("DISCORD_TOKEN")
        .context("'DISCORD_TOKEN' was not found")?;

    let groq_token = secrets
        .get("GROQ_TOKEN")
        .context("'GROQ_TOKEN' was not found")?;

    // Set gateway intents, which decides what events the bot will be notified about
    let intents = GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::MESSAGE_CONTENT
        | GatewayIntents::GUILD_INTEGRATIONS;

    let openai_client = Arc::new(
        ChatGPT::new_with_config(
            groq_token,
            ModelConfigurationBuilder::default()
                .api_url(
                    Url::parse("https://api.groq.com/openai/v1/chat/completions")
                        .context("Failed to parse URL")?,
                )
                .engine(ChatGPTEngine::Custom(
                    "deepseek-r1-distill-llama-70b-specdec",
                ))
                .build()
                .context("Error building OpenAI configuration")?,
        )
        .context("Error initializing OpenAI client")?,
    );

    let discord_client = Client::builder(&discord_token, intents)
        .event_handler(Handler {
            secrets,
            openai_client: openai_client.clone(),
        })
        .await
        .expect("Err creating client");

    Ok(discord_client.into())
}
