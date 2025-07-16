use rand::Rng; // Importa Rng para gerar números aleatórios
use crate::configs::PasswordConfiguration;

const UPPERCASE: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
const LOWERCASE: &str = "abcdefghijklmnopqrstuvwxyz";
const NUMBER: &str = "0123456789";
const SYMBOL: &str = "!@#$%^&*()-_=+[]{}|;:\"'<>,.?/`~";

pub fn generate_password(config: &PasswordConfiguration) -> String {
    let mut characters = String::from(LOWERCASE);

    if config.capital_letters {
        characters.push_str(UPPERCASE);
    }
    if config.numbers {
        characters.push_str(NUMBER);
    }
    if config.symbols {
        characters.push_str(SYMBOL);
    }

    let mut rng = rand::thread_rng();

    (0..config.character_size)
        .map(|_| {
            let idx = rng.gen_range(0..characters.len());
            characters.chars().nth(idx).unwrap()
        })
        .collect()
}