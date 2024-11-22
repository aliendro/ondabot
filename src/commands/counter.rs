use serenity::all::{CreateCommand, ResolvedOption};

pub fn run(_options: &[ResolvedOption]) -> String {
    "WORK IN PROGRESS...".to_owned()
}

pub fn register() -> CreateCommand {
    CreateCommand::new("counter").description("Get counter options for a champion")
}
