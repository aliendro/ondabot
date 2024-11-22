use serenity::all::{
    CommandOptionType, CreateCommand, CreateCommandOption, ResolvedOption, ResolvedValue,
};

pub fn run(options: &[ResolvedOption]) -> String {
    let ascii_text = r#"
……..…../´¯/)……….. (¯\
…………/….//……….. …….\
………../….//………… ….….\
…../´¯/…./´¯………. ./¯ ….¯`\
.././…/…./…./.|……| .….….…...
(.(….(….(…./.)..)..(..(. ….)….)….).)
.……………./…/….. ../………
"#;

    if let Some(ResolvedOption {
        value: ResolvedValue::String(recipient),
        ..
    }) = options.first()
    {
        format!("{ascii_text} \n VAI TOMAR NO CU {}", recipient)
    } else {
        "This is unexpected. Please report to gun gorra.".to_string()
    }
}

pub fn register() -> CreateCommand {
    CreateCommand::new("vtnc")
        .description("Send your best wishes to")
        .add_option(
            CreateCommandOption::new(
                CommandOptionType::String,
                "Destino",
                "quem você deseja agraciar com esta bela mensagem",
            )
            .required(true),
        )
}