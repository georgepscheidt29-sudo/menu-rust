pub struct Permissao {
    leitura: bool,
    escrita: bool,
    execucao: bool
}

impl Permissao {
    pub fn new(leitura: bool, escrita: bool, execucao: bool) -> Permissao {
        Permissao {
            leitura,
            escrita,
            execucao,
        }
    }

    pub fn octal(&self) -> u8 {
        let mut octal: u8 = 0;

        if self.leitura {
            octal += 4
        }
        if self.escrita {
            octal += 2
        }
        if self.execucao {
            octal += 1
        }

        octal
    }
}

pub struct Usuario {
    pub nome: String,
    pub uid: u16,
    pub grupos_ids: Vec<u16>
}
impl Usuario {
    pub fn new(nome: String, uid: u16) -> Usuario {
        let grupos_ids: Vec<u16> = Vec::new();
        Usuario{
            nome,
            uid,
            grupos_ids,
        }
    }
    pub fn adiciona_grupo(&mut self, gid: u16) {
        if self.grupos_ids.contains(&gid) {
            println!("O grupo: {} já existe", gid);
        } else {
            self.grupos_ids.push(gid);
            println!("Grupo {} adicionado", gid);
        }
    }
    pub fn remove_grupo(&mut self, gid: u16) {
        for i in 0..self.grupos_ids.len() {
            if gid == self.grupos_ids[i] {
                self.grupos_ids.remove(i);

                println!("Grupo {} removido de {}", gid, self.nome);

                break
            }
        }
    }
    pub fn listar_grupos(&self) {
        println!("Grupos do usuario {}", self.nome);
        for i in &self.grupos_ids {
            println!("{}", i)
        }
    }
}
pub struct Grupo {
    pub nome: String,
    pub gid: u16,
    pub membros_ids: Vec<u16>
}

impl Grupo {
    pub fn new(nome: String, gid: u16) -> Grupo {
        let membros_ids: Vec<u16> = Vec::new();

        Grupo {
            nome,
            gid,
            membros_ids
        }
    }

    pub fn adiciona_membro(&mut self, uid: u16) {
        if self.membros_ids.contains(&uid) {
            println!("O membro: {} já existe", uid);
        } else {
            self.membros_ids.push(uid);
            println!("Membro {} adicionado", uid);
        }
    }

    pub fn remove_membro(&mut self, uid: u16) {
        for i in 0..self.membros_ids.len() {
            if uid == self.membros_ids[i] {
                self.membros_ids.remove(i);

                println!("Membro {} removido de {}", uid, self.nome);
                break
            }
        }
    }

    pub fn listar_membros(&self) {
        println!("Lista de membros do grupo {}", self.nome);
        for i in &self.membros_ids {
            println!("{}", i);
        }
    }
}

