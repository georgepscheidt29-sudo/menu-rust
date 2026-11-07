// Dupla: George e Humberto

mod file;
mod access;
mod menu;

mod helper;

use std::io;
use crate::menu::{menu_arquivo, menu_diretorio, menu_usuario};

fn main() {
    println!("Bem vindo!");

    let mut exit: bool = false;

    while !exit {

        println!("Selecione uma opção abaixo: \n
        1. Usuarios\n
        2. Grupos\n
        3. Arquivos\n
        4. Diretorios\n
        5. Sair");

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap_or_default();

        let input_as_int: u32 = input.trim().parse().unwrap_or_default();

        match input_as_int {
            1 => menu_usuario(),
            2 => menu_diretorio(),
            3 => menu_arquivo(),
            4 => menu_diretorio(),
            5 => exit = true,
            _ => println!("Opção Invalida, digite um inteiro entre 1 e 5")
        }
    }

    println!("Fechando o programa...");
}
