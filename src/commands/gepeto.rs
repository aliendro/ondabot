use serenity::all::{
    CommandOptionType, CreateCommand, CreateCommandOption, ResolvedOption, ResolvedValue,
};
use tracing::error;

use crate::bot::handler::Handler;

/**
 * TODO: Use streams to avoid memory allocation on every command
 */
pub async fn run<'a>(options: &[ResolvedOption<'a>], handler: &Handler) -> String {
    if let Some(ResolvedOption {
        value: ResolvedValue::String(prompt),
        ..
    }) = options.first()
    {
        let response = handler
            .openai_client
            .clone()
            .send_message(prompt.to_string())
            .await;

        match response {
            Ok(result) => result.message().content.to_string(),
            Err(why) => {
                error!("Failed to execute prompt: {why:#?}");
                "Something went wrong".to_string()
            }
        }
    } else {
        // Fails to parse command option
        "This is unexpected.".to_string()
    }
}

pub fn register() -> CreateCommand {
    CreateCommand::new("gepeto")
        .description("Send a prompt to ChatGPT")
        .add_option(
            CreateCommandOption::new(
                CommandOptionType::String,
                "prompt",
                "Prompt to be sent over to ChatGPT",
            )
            .required(true),
        )
}
