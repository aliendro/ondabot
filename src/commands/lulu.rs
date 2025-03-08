use rand::Rng;
use serenity::all::{CreateCommand, ResolvedOption};

pub fn run(_options: &[ResolvedOption]) -> String {
    let amount = rand::thread_rng().gen_range(2..=100);

    match amount {
        0..25 => format!("Josefa & Josefina seguem tranquilas por hoje."),
        25..50 => format!("O pão de queijo fez efeito. Josefa & Josefina arrancaram {amount} pregas."),
        50..75 => format!("Por que tu comeu requeijão ontem? Josefa & Josefina se irritaram e causaram dano de bleed."),
        75..=100 => format!("Josefa & Josefina resolveram botar a cara no sol."),
        _ => panic!("Impossível tirar tantas pregas.")
    }
}

pub fn register() -> CreateCommand {
    CreateCommand::new("lulu").description("Informa sobre o estado atual de Josefa & Josefina")
}
