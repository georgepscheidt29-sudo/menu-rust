// Dupla: George e Humberto

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

        let input_as_int: u32 = input.trim().parse().unwrap_or_default();

        if input_as_int > 0 && input_as_int < 5 {
            handle_menu(input_as_int);
        } else if input_as_int == 5 {
            exit = true;
        } else {
            println!("Opção Invalida, digite um inteiro de 1 a 5");
        }
    }

    println!("Fechando o programa...");
}
