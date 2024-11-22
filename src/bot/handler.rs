use serenity::all::{
    async_trait, model::gateway::Ready, prelude::*, CreateInteractionResponse,
    CreateInteractionResponseMessage, GuildId, Interaction,
};
use shuttle_runtime::SecretStore;
use tracing::{debug, error, info};

use crate::commands;

pub struct Handler {
    pub secrets: SecretStore,
}

#[async_trait]
impl EventHandler for Handler {
    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        if let Interaction::Command(command) = interaction {
            debug!("Received command interaction: {command:#?}");

            let content = match command.data.name.as_str() {
                "info" => Some(commands::info::run(&command.data.options())),
                "build" => Some(commands::build::run(&command.data.options())),
                "counter" => Some(commands::counter::run(&command.data.options())),
                "pregas" => Some(commands::pregas::run(&command.data.options())),
                "vtnc" => Some(commands::vtnc::run(&command.data.options())),
                _ => Some("not implemented :(".to_string()),
            };

            if let Some(content) = content {
                let data = CreateInteractionResponseMessage::new().content(content);
                let builder = CreateInteractionResponse::Message(data);
                if let Err(why) = command.create_response(&ctx.http, builder).await {
                    error!("Cannot respond to slash command: {why}");
                }
            }
        }
    }

    async fn ready(&self, ctx: Context, ready: Ready) {
        info!("{} is connected!", ready.user.name);

        let somente_onda_id = GuildId::new(
            self.secrets
                .get("SOMENTE_ONDA_ID")
                .expect("'SOMENTE_ONDA_ID' was not found")
                .parse()
                .expect("'SOMENTE_ONDA_ID' must be an integer"),
        );

        let somente_onda_commands = somente_onda_id
            .set_commands(
                &ctx.http,
                vec![
                    commands::info::register(),
                    commands::build::register(),
                    commands::counter::register(),
                    commands::vtnc::register(),
                ],
            )
            .await;

        info!("Registered SOMENTE_ONDA commands!");
        debug!("SOMENTE_ONDA commands: {somente_onda_commands:#?}");

        let cml_id = GuildId::new(
            self.secrets
                .get("CML_ID")
                .expect("'CML_ID' was not found")
                .parse()
                .expect("'CML_ID' must be an integer"),
        );

        let cml_commands = cml_id
            .set_commands(
                &ctx.http,
                vec![commands::pregas::register(), commands::vtnc::register()],
            )
            .await;

        info!("Registered CML commands!");
        debug!("CML commands: {cml_commands:#?}");
    }
}
