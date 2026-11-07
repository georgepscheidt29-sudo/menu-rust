use std::io;
use crate::access::Permissao;

pub fn escolher_permissao() -> (Permissao, Permissao, Permissao){
    let mut permissoes_vec: [[bool;3];3] = [[false;3];3];
    let seq: [&str; 3] = ["Dono", "Grupo", "Outros"];
    let perms: [&str; 3] = ["Leitura", "Escrita", "Execução"];

    for i in 0..3 {
        println!("Definindo a permissão para {}", seq[i]);
        let mut perm_atual: [bool; 3] = [false; 3];

        for j in 0..3 {
            println!("Digite 1 para garantir permissão de {}, qualquer outra tecla para negar", perms[j]);
            let mut input: String = "".to_string();

            io::stdin().read_line(&mut input).unwrap_or_default();

            input = input.trim().to_string();

            if input == "1" {
                perm_atual[j] = true;
            } else {
                perm_atual[j] = false;
            }
        }

        permissoes_vec[i] = perm_atual;

    }

    (
     Permissao::new(permissoes_vec[0][0], permissoes_vec[0][1], permissoes_vec[0][2]),
     Permissao::new(permissoes_vec[1][0], permissoes_vec[1][1], permissoes_vec[1][2]),
     Permissao::new(permissoes_vec[2][0], permissoes_vec[2][1], permissoes_vec[2][2])
    )
}