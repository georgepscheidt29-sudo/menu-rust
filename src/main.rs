mod file;
mod access;
mod menu;

mod helper;

use std::io;
use crate::menu::handle_menu;

fn main() {
    println!("Bem vindo!");

    let mut exit: bool = false;

    while !exit {

        println!("Selecione uma opção abaixo: \n
        1. Criar Novo Objeto\n
        2. Atualizar Objeto Existente\n
        3. Deletar Objeto\n
        4. Ler Objetos\n
        5. Sair");

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap_or_default();

        let input_ref = input.trim();

        match input_ref {
            "1" => handle_menu(input_ref),
            "2" => handle_menu(input_ref),
            "3" => handle_menu(input_ref),
            "4" => handle_menu(input_ref),
            "5" => exit = true,
            _ => println!("Opção invalida, por favor selecione novamente.")
        }
    }

    println!("Exiting...");
}
