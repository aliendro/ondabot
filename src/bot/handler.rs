use crate::commands;
use chatgpt::client::ChatGPT;
use serenity::all::*;
use shuttle_runtime::SecretStore;
use std::{sync::Arc, time::Duration};
use tracing::{debug, error, info};
use unicode_segmentation::UnicodeSegmentation;

pub struct Handler {
    pub secrets: SecretStore,
    pub openai_client: Arc<ChatGPT>,
}

#[async_trait]
impl EventHandler for Handler {
    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        let Interaction::Command(command) = interaction else {
            return;
        };

        debug!("Received command interaction: {command:#?}");

        let msg = CreateInteractionResponseMessage::new().content("Pensando...");
        let builder = CreateInteractionResponse::Defer(msg);
        if let Err(why) = command.create_response(&ctx.http, builder).await {
            error!("Failed to generate interaction response: {why}");
        }

        let content = match command.data.name.as_str() {
            "gepeto" => Some(commands::gepeto::run(&command.data.options(), &self).await),
            "vtnc" => Some(commands::vtnc::run(&command.data.options())),
            "pregas" => Some(commands::pregas::run(&command.data.options())),
            _ => Some("not implemented :(".to_string()),
        };

        let Some(content) = content else {
            return;
        };

        let contents: Vec<CreateEmbed> = content
            .graphemes(true)
            .collect::<Vec<&str>>()
            .chunks(2000)
            .map(|chunk| chunk.concat())
            .map(|m| CreateEmbed::new().description(m))
            .collect();

        let builder = if command.data.name.as_str() == "pregas" {
            let poll = CreatePoll::new()
                .question("Como você avalia a precisão da tirada de pregas?")
                .answers(vec![
                    CreatePollAnswer::new().text("Tirou foi pouco"),
                    CreatePollAnswer::new().text("E ainda tinha prega?"),
                    CreatePollAnswer::new().text("Arranca de novo"),
                ])
                .duration(Duration::new(3_600, 0));

            CreateInteractionResponseFollowup::new()
                .add_embeds(contents)
                .poll(poll)
        } else {
            CreateInteractionResponseFollowup::new().add_embeds(contents)
        };

        if let Err(why) = command.create_followup(&ctx.http, builder).await {
            error!("Failed to send message in channel: {why}");
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
                    commands::pregas::register(),
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
