use std::io;
use crate::access::{Grupo, Usuario};
use crate::file::{Arquivo, Diretorio};
use crate::helper::{criacao_grupo, criacao_usuario, entrar_grupo, listar_grupos, listar_usuarios, remove_usuario, remover_grupos, sair_grupo};

pub fn menu_usuario(usuarios: &mut Vec<Usuario>, grupos: &mut Vec<Grupo>) {

    let mut input = String::new();
        println!("Escolha uma opção:
    1. Criar novo Usuario
    2. Adicionar Usuario a um grupo
    3. Remover Usuario de um grupo
    4. Listar Usuarios existentes
    5. Remover Usuarios existentes
    6. Retornar ao menu inicial
    ");

    io::stdin().read_line(&mut input).unwrap_or_default();

    let input_as_int: u32 = input.trim().parse().unwrap_or_default();

    let proximo_uid: u16 = (usuarios.len() + 1) as u16;
    match input_as_int {
        1 => usuarios.push(criacao_usuario(proximo_uid)),
        2 => entrar_grupo(usuarios, grupos),
        3 => sair_grupo(usuarios, grupos),
        4 => listar_usuarios(usuarios),
        5 => remove_usuario(usuarios, grupos),
        6 => println!(),
        _ => println!("Opção Invalida")
    }


}

pub fn menu_grupo(grupos: &mut Vec<Grupo>, usuarios: &mut Vec<Usuario>) {
    let mut input = String::new();
    println!("Escolha uma opção:
    1. Criar novo Grupo
    2. Listar Grupos existentes
    3. Remover Grupos existentes
    4. Retornar ao menu inicial
    ");

    io::stdin().read_line(&mut input).unwrap_or_default();

    let input_as_int: u32 = input.trim().parse().unwrap_or_default();

    let proximo_gid: u16 = (grupos.len() + 1) as u16;
    match input_as_int {
        1 => grupos.push(criacao_grupo(proximo_gid)),
        2 => listar_grupos(grupos),
        3 => remover_grupos(usuarios, grupos),
        4 => println!(),
        _ => println!("Opção Invalida")
    }
}

pub fn menu_arquivo(arquivos: &Vec<Arquivo>) {

}

pub fn menu_diretorio(diretorios: &Vec<Diretorio>) {

}

