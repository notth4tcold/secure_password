use std::io::{self, Write}; // Importa Write para usar .flush()
use rand::Rng; // Importa Rng para gerar números aleatórios

fn main() {
    println!();
    println!("=== Gerador de Senhas Seguras ===");
    println!();

    let character_size = get_character_size();
    let capital_letters = get_capital_letters();
    let numbers = get_numbers();
    let symbols = get_symbols();

    let mut characters = String::new();
    characters.push_str("abcdefghijklmnopqrstuvwxyz");

    if capital_letters {
        characters.push_str("ABCDEFGHIJKLMNOPQRSTUVWXYZ");
    }
    if numbers {
        characters.push_str("0123456789");
    }
    if symbols {
        characters.push_str("!@#$%^&*()-_=+[]{}|;:\"'<>,.?/`~");
    }

    let mut rng = rand::thread_rng();
    let password: String = (0..character_size)
        .map(|_| {
            let idx = rng.gen_range(0..characters.len());
            characters.chars().nth(idx).unwrap()
        })
        .collect();


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