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
    let mut usuarios: Vec<Usuario> = Vec::new();
    let mut grupos: Vec<Grupo> = Vec::new();
    let mut arquivos: Vec<Arquivo> = Vec::new();
    let mut diretorios: Vec<Diretorio> = Vec::new();

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
            1 => menu_usuario(&mut usuarios, &mut grupos),
            2 => menu_grupo(&mut grupos, &mut usuarios),
            3 => menu_arquivo(&mut arquivos, &mut grupos, &mut usuarios),
            4 => menu_diretorio(&mut diretorios, &mut arquivos, &mut usuarios),
            5 => exit = true,
            _ => println!("Opção Invalida, digite um inteiro entre 1 e 5")
        }
    }

    println!("Fechando o programa...");
}
