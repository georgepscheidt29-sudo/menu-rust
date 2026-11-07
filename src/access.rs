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
    pub fn adicionar_grupo(&mut self, gid: u16) {
        if self.grupos_ids.contains(&gid) {
            println!("O grupo: {} já existe", gid);
        } else {
            self.grupos_ids.push(gid);
            println!("Grupo {} adicionado", gid);
        }
    }
    pub fn remove_grupo(&mut self, gid: u16) {
        for i in 0..self.grupos_ids.len() {
            if usize::from(gid) == i {
                self.grupos_ids.remove(i);
            }
        }
    }
    pub fn listar_grupos(&self) {
        for y in self.grupos_ids {
            println!("Lista de grupos existentes {}", self.grupos_ids)
        }
    }
}
pub struct Grupo {
    nome: String,
    gid: u16,
    membros_ids: Vec<u16>
}

