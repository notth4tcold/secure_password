use colored::*;
use std::io::{self, Write}; // Importa Write para usar .flush()

pub fn get_character_size() -> i32 {
    let mut character_size = String::new();
    print!("{}", "Quantos caracteres? ".bold().blue());
    io::stdout().flush().unwrap();

    io::stdin()
        .read_line(&mut character_size)
        .expect("Falha ao ler a entrada");

    match character_size.trim().parse() {
        Ok(size) => size,
        Err(_) => {
            println!("{}", "⚠️ Valor inválido. Usando 0.".red());
            0 // Retorna 0 em caso de erro
        }
    }
}

pub fn get_capital_letters() -> bool {
    prompt_yes_no("Incluir letras maiúsculas? (s/n): ")
}

pub fn get_numbers() -> bool {
    prompt_yes_no("Incluir números? (s/n): ")
}

pub fn get_symbols() -> bool {
    prompt_yes_no("Incluir símbolos? (s/n): ")
}

fn prompt_yes_no(prompt: &str) -> bool {
    let mut input = String::new();
    print!("{}", prompt.bold().blue());
    io::stdout().flush().unwrap();

    io::stdin()
        .read_line(&mut input)
        .expect("Falha ao ler a entrada");

    input.trim().to_lowercase() == "s"
}
