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
    nome: String,
    uid: u16,
    grupos_ids: Vec<u16>
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
        let mut removido: bool = false;
        for i in 0..self.grupos_ids.len() {
            if gid == self.grupos_ids[i] {
                self.grupos_ids.remove(i);

                println!("Membro {} removido de {}", gid, self.nome);
                removido = true;
            }
        }

        if !removido {
            println!("Usuario {} já não pertence ao grupo {}", gid, self.nome);
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
    nome: String,
    gid: u16,
    membros_ids: Vec<u16>
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
        self.membros_ids.push(uid);
    }

    pub fn remove_membro(&mut self, uid: u16) {
        let mut removido: bool = false;
        for i in 0..self.membros_ids.len() {
            if uid == self.membros_ids[i] {
                self.membros_ids.remove(i);

                println!("Membro {} removido de {}", uid, self.nome);
                removido = true;
            }
        }
        if !removido {
            println!("Usuario {} já não pertence ao grupo {}", uid, self.nome);
        }
    }

    pub fn listar_membros(&self) {
        println!("Lista de membros do grupo {}", self.nome);
        for i in &self.membros_ids {
            println!("{}", i);
        }
    }
}

