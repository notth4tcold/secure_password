use std::io::{self, Write}; // Importa Write para usar .flush()
use rand::Rng; // Importa Rng para gerar números aleatórios

struct PasswordConfiguration {
    character_size: i32,
    capital_letters: bool,
    numbers: bool,
    symbols: bool,
}

const UPPERCASE: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
const LOWERCASE: &str = "abcdefghijklmnopqrstuvwxyz";
const NUMBER: &str = "0123456789";
const SYMBOL: &str = "!@#$%^&*()-_=+[]{}|;:\"'<>,.?/`~";

fn main() {
    println!();
    println!("=== Gerador de Senhas Seguras ===");
    println!();

    let config = PasswordConfiguration {
        character_size: get_character_size(),
        capital_letters: get_capital_letters(),
        numbers: get_numbers(),
        symbols: get_symbols(),
    };

    let password: String = generate_password(&config);

    println!();
    println!("🔐 Sua senha segura: {}", password);
}

fn get_character_size() -> i32 {
    let mut character_size = String::new();
    print!("Quantos caracteres? ");
    io::stdout().flush().unwrap();

    io::stdin()
        .read_line(&mut character_size)
        .expect("Falha ao ler a entrada");

    match character_size.trim().parse() {
        Ok(size) => size,
        Err(_) => {
            println!("Por favor, insira um número válido.");
            0 // Retorna 0 em caso de erro
        }
    }
}

fn get_capital_letters() -> bool {
    let mut capital_letters = String::new();
    print!("Incluir letras maiúsculas? (s/n): ");
    io::stdout().flush().unwrap();

    io::stdin()
        .read_line(&mut capital_letters)
        .expect("Falha ao ler a entrada");

    capital_letters.trim().to_lowercase() == "s"
}

fn get_numbers() -> bool {
    let mut numbers = String::new();
    print!("Incluir números? (s/n): ");
    io::stdout().flush().unwrap();

    io::stdin()
        .read_line(&mut numbers)
        .expect("Falha ao ler a entrada");

    numbers.trim().to_lowercase() == "s"
}

fn get_symbols() -> bool {
    let mut symbols = String::new();
    print!("Incluir símbolos? (s/n): ");
    io::stdout().flush().unwrap();

    io::stdin()
        .read_line(&mut symbols)
        .expect("Falha ao ler a entrada");

    symbols.trim().to_lowercase() == "s"
}

fn generate_password(config: &PasswordConfiguration) -> String {
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