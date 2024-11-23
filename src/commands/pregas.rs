use rand::Rng;
use serenity::all::{CreateCommand, ResolvedOption};

pub fn run(_options: &[ResolvedOption]) -> String {
    format!(
        "Você tirou {} pregas do Kamikaze.",
        rand::thread_rng().gen_range(2..=100)
    )
}

pub fn register() -> CreateCommand {
    CreateCommand::new("pregas").description("Performs a prega check on Kamikaze")
}
