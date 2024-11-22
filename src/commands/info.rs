use serenity::all::{CreateCommand, ResolvedOption};

pub fn run(_options: &[ResolvedOption]) -> String {
    "WORK IN PROGRESS...".to_owned()
}

pub fn register() -> CreateCommand {
    CreateCommand::new("info").description("Get general information for a champion")
}
