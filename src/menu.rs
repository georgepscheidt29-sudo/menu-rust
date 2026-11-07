use std::io;

pub fn handle_menu(mode: u32) {
    let mut acao: String = String::new();
    let mut input: String = String::new();
    match mode {
        1 => acao = String::from("criado"),
        2 => acao = String::from("atualizado"),
        3 => acao = String::from("deletado"),
        _ => acao = String::from("lido"),
    }

    println!("Selecione um objeto a ser {}:\n
    1. Arquivo\
    2. Diretorio\
    3. Usuario\
    4. Grupo\n\
    Pressione Enter ou qualquer outro valor para voltar ao menu inicial", {acao});

    io::stdin().read_line(&mut input).unwrap_or_default();

    let input_ref = input.trim();

    match input_ref {
        "1" => unimplemented!(),
        "2" => unimplemented!(),
        "3" => unimplemented!(),
        "4" => unimplemented!(),
        _ => (),
    }

}

