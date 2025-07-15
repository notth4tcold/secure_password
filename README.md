# 🔐 Gerador de Senhas Seguras (Rust CLI)

Um projetinho simples, mas poderoso, feito em Rust, que gera senhas aleatórias seguras com base nas preferências do usuário. Ideal para quem está começando com Rust e quer praticar lógica, controle de fluxo, entrada/saída, modularização, e o uso de crates da comunidade.

---

## 🚀 Funcionalidades

- Escolha o tamanho da senha
- Opções para incluir:
  - Letras maiúsculas
  - Letras minúsculas
  - Números
  - Símbolos especiais
- Interface amigável via terminal
- Geração de senhas aleatórias seguras
- Código modular e limpo com boas práticas

---

## 🛠️ Conceitos Praticados

- Entrada e saída com `std::io`
- Controle de fluxo com `if`, `match`, `loop`, etc.
- Uso de `struct`, `enum`, e `Result`
- Modularização com `mod` e `lib.rs`
- Manipulação de strings e vetores
- Geração de números aleatórios com o crate `rand`
- Criação e execução de projetos com `cargo`

---

## 📦 Dependências

Este projeto utiliza os seguintes crates:

- [`rand`](https://crates.io/crates/rand): Para gerar os caracteres aleatórios
- [`colored`](https://crates.io/crates/colored) _(opcional)_: Para deixar a saída mais colorida e estilosa
- [`clipboard`](https://crates.io/crates/clipboard) _(opcional)_: Para copiar a senha direto para a área de transferência

Adicione ao `Cargo.toml`:

```toml
[dependencies]
rand = "0.8"
colored = "2.0"        # opcional
clipboard = "0.7"      # opcional
```

---

## 📚 Como Rodar

```toml
git clone https://github.com/notth4tcold/secure_password.git
cd secure_password
cargo run
```

---

## ✨ Futuras Melhorias (para treinar mais ainda!)

- Salvar senhas em um arquivo (std::fs)
- Copiar senha automaticamente para o clipboard
- Adicionar testes automatizados
- Adicionar modo interativo com menus (usando dialoguer ou inquire)
- Suporte a múltiplas senhas por vez
- Interface gráfica com egui ou gtk-rs (nível avançado)
