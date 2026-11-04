// Menu interativo
/*
Seu trabalho deverá criar um menu interativo (via terminal) que possibilite manipular: criar, editar,
apagar e listar cada uma das estruturas implementadas. O objetivo prático é consolidar a utilização
correta do Borrowing no Rust. Utilize as Tabelas 1 e 2 como referência para a implementação. Lembre-se
de utilizar o bloco impl para funções associadas
Diretamente copiado do pdf do trabalho para facilitar implementação
TODO: Apagar antes de enviar
*/

use std::io;

fn main() {
    println!("Bem vindo!");

    let mut exit: bool = false;

    while !exit {
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap_or_default();
    }
}
