pub struct Permissao {
    leitura: bool,
    escrita: bool,
    execucao: bool
}

pub struct Usuario {
    nome: String,
    uid: u16,
    grupos_ids: Vec<u16>
}

pub struct Grupo {
    nome: String,
    gid: u16,
    membros_ids: Vec<u16>
}
