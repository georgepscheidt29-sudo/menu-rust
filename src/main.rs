// Dupla: George e Humberto

mod file;
mod access;
mod menu;

mod helper;

use std::io;
use crate::access::{Grupo, Usuario};
use crate::file::{Arquivo, Diretorio};
use crate::menu::{menu_arquivo, menu_diretorio, menu_grupo, menu_usuario};


fn main() {
    let mut USUARIOS: Vec<Usuario> = Vec::new();
    let mut GRUPOS: Vec<Grupo> = Vec::new();
    let mut ARQUIVOS: Vec<Arquivo> = Vec::new();
    let mut DIRETORIOS: Vec<Diretorio> = Vec::new();

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
            1 => menu_usuario(&mut USUARIOS, &mut GRUPOS),
            2 => menu_grupo(&mut GRUPOS),
            3 => menu_arquivo(&mut ARQUIVOS),
            4 => menu_diretorio(&mut DIRETORIOS),
            5 => exit = true,
            _ => println!("Opção Invalida, digite um inteiro entre 1 e 5")
        }
    }

    println!("Fechando o programa...");
}
