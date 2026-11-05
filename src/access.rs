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

pub struct Grupo {
    nome: String,
    gid: u16,
    membros_ids: Vec<u16>
}

