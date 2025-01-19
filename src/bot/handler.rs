use std::sync::Arc;

use crate::commands;
use chatgpt::client::ChatGPT;
use serenity::all::{
    async_trait, model::gateway::Ready, prelude::*, CreateInteractionResponse,
    CreateInteractionResponseMessage, GuildId, Interaction,
};
use shuttle_runtime::SecretStore;
use tracing::{debug, error, info};
use unicode_segmentation::UnicodeSegmentation;

pub struct Handler {
    pub secrets: SecretStore,
    pub openai_client: Arc<ChatGPT>,
}

#[async_trait]
impl EventHandler for Handler {
    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        if let Interaction::Command(command) = interaction {
            debug!("Received command interaction: {command:#?}");

            let content = match command.data.name.as_str() {
                "gepeto" => Some(commands::gepeto::run(&command.data.options(), &self).await),
                "vtnc" => Some(commands::vtnc::run(&command.data.options())),
                "pregas" => Some(commands::pregas::run(&command.data.options())),
                _ => Some("not implemented :(".to_string()),
            };

            if let Some(content) = content {
                let mut contents: Vec<String> = Vec::new();
                if content.len() > 2000 {
                    content
                        .graphemes(true)
                        .collect::<Vec<&str>>()
                        .chunks(2000)
                        .map(|chunk| chunk.concat())
                        .for_each(|message| contents.push(message));
                } else {
                    contents.push(content);
                }

                for message in contents {
                    println!("{message}");
                    let data = CreateInteractionResponseMessage::new().content(message);
                    let builder = CreateInteractionResponse::Message(data);
                    if let Err(why) = command.create_response(&ctx.http, builder).await {
                        error!("Cannot respond to slash command: {why}");
                    }
                }
            }
        }
    }

    async fn ready(&self, ctx: Context, ready: Ready) {
        info!("{} is connected!", ready.user.name);

        let cml_id = GuildId::new(
            self.secrets
                .get("CML_ID")
                .expect("'CML_ID' was not found")
                .parse()
                .expect("'CML_ID' must be an integer"),
        );

        let commands = cml_id
            .set_commands(
                &ctx.http,
                vec![
                    commands::gepeto::register(),
                    commands::vtnc::register(),
                    // commands::pregas::register(),
                ],
            )
            .await;

        match commands {
            Ok(commands) => {
                info!("Registered CML commands!");
                debug!("CML commands: {commands:#?}");
            }
            Err(why) => error!("Failed to register commands: {why:?}"),
        }
    }
}
