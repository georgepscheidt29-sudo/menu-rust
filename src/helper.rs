use std::io;
use crate::access::{Grupo, Permissao, Usuario};

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

pub fn criacao_usuario(uid: u16) -> Usuario {

    let mut input_nome = String::new();
    println!("Digite o nome do novo usuario");
    io::stdin().read_line(&mut input_nome).unwrap_or_default();

    Usuario::new(input_nome, uid)
}

pub fn entrar_grupo(usuarios: &mut Vec<Usuario>, grupos: &mut Vec<Grupo>) {
    let id_usuario = confirmar_usuario(usuarios);
    let id_grupo = confirmar_grupo(grupos);
    let mut index: [usize; 2] = [0,0];
    
    if id_usuario.0 < 0 || id_grupo.0 < 0 {
        println!("Usuario ou Grupo invalido");
        return;
    } else {
        index[0] = id_usuario.0 as usize;
        index[1] = id_grupo.0 as usize;
        
        usuarios[index[0]].adiciona_grupo(id_grupo.1);
        grupos[index[1]].adiciona_membro(id_grupo.1);
    }
}

pub fn sair_grupo(usuarios: &mut Vec<Usuario>, grupos: &mut Vec<Grupo>) {
    let id_usuario = confirmar_usuario(usuarios);
    let id_grupo = confirmar_grupo(grupos);
    let mut index: [usize; 2] = [0,0];

    if id_usuario.0 < 0 || id_grupo.0 < 0 {
        println!("Usuario ou Grupo invalido");
        return;
    } else {
        index[0] = id_usuario.0 as usize;
        index[1] = id_grupo.0 as usize;

        usuarios[index[0]].remove_grupo(id_grupo.1);
        grupos[index[1]].remove_membro(id_grupo.1);
    }
}

fn confirmar_usuario(usuarios: &mut Vec<Usuario>) -> (i16, u16) {
    let mut input_usuario = String::new();
    println!("Digite o id do usuario: ");
    io::stdin().read_line(&mut input_usuario).unwrap_or_default();

    let input_uid: u16 = input_usuario.trim().parse().unwrap_or_default();
    let mut u_existe: bool = false;

    let mut usuario: usize = 0;

    for i in 0..usuarios.len() {
        if usuarios[i].uid == input_uid {
            u_existe = true;
            usuario = i;
        }
    }
    
    if u_existe {
        (usuario as i16, input_uid)
    } else {
        (-1, input_uid)
    }
}

fn confirmar_grupo(grupos: &mut Vec<Grupo>) -> (i16, u16) {

    let mut input_grupo = String::new();
    println!("Digite o id do grupo: ");
    io::stdin().read_line(&mut input_grupo).unwrap_or_default();
    
    let input_gid: u16 = input_grupo.trim().parse().unwrap_or_default();
    
    let mut g_existe: bool = false;
    
    let mut grupo: usize = 0;
    
    for i in 0..grupos.len() {
        if grupos[i].gid == input_gid {
            g_existe = true;
            grupo = i;
        }
    }

    if g_existe {
        (grupo as i16, input_gid)
    } else {
        (-1, input_gid)
    }
}

pub fn listar_usuarios(usuarios: &Vec<Usuario>,) {
    for i in 0..usuarios.len() {
        println!("{}, uid: {}", usuarios[i].nome, usuarios[i].uid);
    }
}

pub fn remove_usuario(usuarios: &mut Vec<Usuario>, grupos: &mut Vec<Grupo>) {
    let id_usuario = confirmar_usuario(usuarios);
    let index: usize;
    
    if id_usuario.0 < 0 {
        println!("Usuario invalido");
        return;
    } else {
        index = id_usuario.0 as usize;
        
        for i in 0..usuarios[index].grupos_ids.len() {
            for j in 0..grupos.len() {
                if usuarios[index].grupos_ids[i] == grupos[j].gid {
                    grupos[j].remove_membro(id_usuario.1);
                }
                
            }
        }
        
        usuarios.remove(index);
    }
}

pub fn criacao_grupo(gid: u16) -> Grupo {
    let mut input_nome = String::new();
    println!("Digite o nome do novo grupo");
    io::stdin().read_line(&mut input_nome).unwrap_or_default();

    Grupo::new(input_nome, gid)
}

pub fn listar_grupos(grupos: &Vec<Grupo>) {
    for i in 0..grupos.len() {
        println!("{}, gid: {}", grupos[i].nome, grupos[i].gid)
    }
}

pub fn remover_grupos(usuarios: &mut Vec<Usuario>, grupos: &mut Vec<Grupo>) {
    let id_grupo = confirmar_grupo(grupos);
    let index: usize;

    if id_grupo.0 < 0 {
        println!("Usuario invalido");
        return;
    } else {
        index = id_grupo.0 as usize;
        for i in 0..grupos[index].membros_ids.len() {
            for j in 0..grupos.len() {
                if grupos[index].membros_ids[i] == usuarios[j].uid {
                    usuarios[j].remove_grupo(id_grupo.1);
                }
            }
        }
    }
}