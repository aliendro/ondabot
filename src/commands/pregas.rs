use rand::Rng;
use serenity::all::{CreateCommand, ResolvedOption};

pub fn run(_options: &[ResolvedOption]) -> String {
    let amount = rand::thread_rng().gen_range(2..=100);

    match amount {
        0..25 => format!("Um dia comum para o Kamikaze. Você levou {amount} pregas pra casa."),
        25..50 => format!("Apesar da intensa resistência, Kamikaze te presentou com {amount} pregas."),
        50..75 => format!("O gerente enlouqueceu! Kamikaze distribuiu {amount} pregas pra você e toda sua família."),
        75..=100 => format!("CRITICAL HIT! Você arrancou {amount} pregas do Kamikaze!"), 
        _ => format!("A quantidade de pregas foi tão grande que o bot não conseguiu calcular com precisão.")
    }
}

pub fn register() -> CreateCommand {
    CreateCommand::new("pregas").description("Performs a prega check on Kamikaze")
}
