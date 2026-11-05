// Menu interativo
/*
Seu trabalho deverá criar um menu interativo (via terminal) que possibilite manipular: criar, editar,
apagar e listar cada uma das estruturas implementadas. O objetivo prático é consolidar a utilização
correta do Borrowing no Rust. Utilize as Tabelas 1 e 2 como referência para a implementação. Lembre-se
de utilizar o bloco impl para funções associadas
Diretamente copiado do pdf do trabalho para facilitar implementação
TODO: Apagar antes de enviar
*/
mod file;
mod access;
mod menu;

use std::io;

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
            "1" => unimplemented!(),
            "2" => unimplemented!(),
            "3" => unimplemented!(),
            "4" => unimplemented!(),
            "5" => exit = true,
            _ => println!("Opção invalida, por favor selecione novamente.")
        }
    }

    println!("Exiting...");
}
