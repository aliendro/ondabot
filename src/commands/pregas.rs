use rand::Rng;
use serenity::all::{CreateCommand, ResolvedOption};

pub fn run(_options: &[ResolvedOption]) -> String {
    format!(
        "Kamikaze perdeu {} pregas hoje.",
        rand::thread_rng().gen_range(2..=20)
    )
}

pub fn register() -> CreateCommand {
    CreateCommand::new("pregas")
        .description("Retrieves damage estimation on Kamikaze's brioco")
        .description_localized(
            "pt-BR",
            "Retorna uma estimativa das pregas perdidas do Kamikaze",
        )
}
