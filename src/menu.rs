use std::io;
use crate::access::{Grupo, Usuario};
use crate::file::{Arquivo, Diretorio};
use crate::helper::{adicionar_arquivo_diretorio, criacao_arquivo, criacao_diretorio, criacao_grupo, criacao_usuario, detalhes_arquivo, detalhes_grupo, detalhes_usuario, entrar_grupo, listar_conteudo_diretorio, listar_grupos, listar_usuarios, remove_usuario, remover_arquivo, remover_arquivo_diretorio, remover_diretorio, remover_grupos, sair_grupo, trocar_permissao};

pub fn menu_usuario(usuarios: &mut Vec<Usuario>, grupos: &mut Vec<Grupo>) {

    let mut input = String::new();
        println!("Escolha uma opção:
    1. Criar novo Usuario
    2. Adicionar Usuario a um grupo
    3. Remover Usuario de um grupo
    4. Listar Usuarios existentes
    5. Remover Usuarios existentes
    6. Ver detalhes de um Usuario
    7. Retornar ao menu inicial
    ");

    io::stdin().read_line(&mut input).unwrap_or_default();

    let input_as_int: u32 = input.trim().parse().unwrap_or_default();

    let mut proximo_uid: u16 = 1;
    for i in 0..usuarios.len() {
        if usuarios[i].uid >= proximo_uid {
            proximo_uid = usuarios[i].uid + 1;
        }
    }
    match input_as_int {
        1 => usuarios.push(criacao_usuario(proximo_uid)),
        2 => entrar_grupo(usuarios, grupos),
        3 => sair_grupo(usuarios, grupos),
        4 => listar_usuarios(usuarios),
        5 => remove_usuario(usuarios, grupos),
        6 => detalhes_usuario(usuarios),
        7 => println!(),
        _ => println!("Opção Invalida")
    }


}

pub fn menu_grupo(grupos: &mut Vec<Grupo>, usuarios: &mut Vec<Usuario>) {
    let mut input = String::new();
    println!("Escolha uma opção:
    1. Criar novo Grupo
    2. Listar Grupos existentes
    3. Remover Grupos existentes
    4. Ver detalhes de um Grupo
    5. Retornar ao menu inicial
    ");

    io::stdin().read_line(&mut input).unwrap_or_default();

    let input_as_int: u32 = input.trim().parse().unwrap_or_default();

    let mut proximo_gid: u16 = 1;
    for i in 0..grupos.len() {
        if grupos[i].gid >= proximo_gid {
            proximo_gid = grupos[i].gid + 1;
        }
    }
    match input_as_int {
        1 => grupos.push(criacao_grupo(proximo_gid)),
        2 => listar_grupos(grupos),
        3 => remover_grupos(usuarios, grupos),
        4 => detalhes_grupo(grupos),
        5 => println!(),
        _ => println!("Opção Invalida")
    }
}

pub fn menu_arquivo(arquivos: &mut Vec<Arquivo>, grupos: &mut Vec<Grupo>, usuarios: &mut Vec<Usuario>) {
    let mut input = String::new();
    println!("Escolha uma opção:
    1. Criar novo Arquivo
    2. Alterar Permissões
    3. Ver detalhes de um Arquivo
    4. Remover Arquivo
    5. Retornar ao menu inicial
    ");

    io::stdin().read_line(&mut input).unwrap_or_default();

    let input_as_int: u32 = input.trim().parse().unwrap_or_default();

    match input_as_int {
        1 => criacao_arquivo(usuarios, grupos, arquivos),
        2 => trocar_permissao(arquivos),
        3 => detalhes_arquivo(arquivos),
        4 => remover_arquivo(arquivos),
        5 => println!(),
        _ => println!("Opção Invalida")
    }
}

pub fn menu_diretorio(diretorios: &mut Vec<Diretorio>, arquivos: &mut Vec<Arquivo>, usuarios: &mut Vec<Usuario>) {
    let mut input = String::new();
    println!("Escolha uma opção:
    1. Criar novo Diretorio
    2. Adicionar Arquivo ao Diretorio
    3. Remover Arquivo do Diretorio
    4. Remover Diretorio
    5. Listar conteudo de um Diretorio
    6. Retornar ao menu inicial
    ");

    io::stdin().read_line(&mut input).unwrap_or_default();

    let input_as_int: u32 = input.trim().parse().unwrap_or_default();

    match input_as_int {
        1 => criacao_diretorio(usuarios, diretorios),
        2 => adicionar_arquivo_diretorio(diretorios, arquivos),
        3 => remover_arquivo_diretorio(diretorios),
        4 => remover_diretorio(diretorios),
        5 => listar_conteudo_diretorio(diretorios),
        6 => println!(),
        _ => println!("Opção Invalida")
    }
}

