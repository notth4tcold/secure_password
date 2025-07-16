mod configs;
mod generator;
mod input;

use colored::*;
use configs::PasswordConfiguration;
use generator::generate_password;
use input::{get_character_size, get_capital_letters, get_numbers, get_symbols};

fn main() {
    println!();
    println!("{}", "=== Gerador de Senhas Seguras ===".red().cyan());
    println!();

    let config = PasswordConfiguration {
        character_size: get_character_size(),
        capital_letters: get_capital_letters(),
        numbers: get_numbers(),
        symbols: get_symbols(),
    };

    let password: String = generate_password(&config);

    println!();
    println!("{} {}", "🔐 Sua senha segura:".green().bold(), password.yellow().bold());
}