use crate::access::Permissao;
use std::io;
pub struct Arquivo {
    nome: String,
    tamanho: u64,
    permissoes: (Permissao, Permissao, Permissao),
    uid_dono: u16,
    gid_grupo: u16

}

impl Arquivo {
    pub fn new(nome: String, tamanho: u64, uid: u16, gid: u16) -> Arquivo {
        let perm_dono: Permissao = Permissao::new(true, true, false);
        let perm_grupo: Permissao = Permissao::new(true, true, false);
        let perm_outros: Permissao = Permissao::new(true, false, false);

        Arquivo {
            nome,
            tamanho,
            permissoes: (perm_dono, perm_grupo, perm_outros),
            uid_dono: uid,
            gid_grupo: gid

        }
    }

    pub fn alterar_permissao(&mut self, permissoes: (Permissao, Permissao, Permissao)) {
        self.permissoes = permissoes;
    }

    pub fn stat(&self) {
        let mut octal: String = String::from("");
        octal.push_str(&self.permissoes.0.octal().to_string());
        octal.push_str(&self.permissoes.1.octal().to_string());
        octal.push_str(&self.permissoes.2.octal().to_string());

        println!("Arquivo: {}", self.nome);
        println!("Tamanho: {} bytes", self.tamanho);
        println!("Permissões: {octal}");
        println!("Uid: {}", self.uid_dono);
        println!("Gid: {}", self.gid_grupo);
    }
}

pub struct Diretorio {
    nome: String,
    arquivos: Vec<Arquivo>,
    permissoes: (Permissao, Permissao, Permissao),
    uid_dono: u16
}

impl Diretorio {
    pub fn new(nome: String, permissoes: (Permissao, Permissao, Permissao), uid_dono: u16) -> Diretorio {
        let arquivos: Vec<Arquivo> = Vec::new();

        Diretorio {
            nome,
            arquivos,
            permissoes,
            uid_dono
        }
    }

    pub fn adiciona_arquivo(&mut self, arquivo: Arquivo) {
        self.arquivos.push(arquivo);
    }

    pub fn remove_arquivo(&mut self, nome_arquivo: String) {
        let mut removido: bool = false;
        for i in 0..self.arquivos.len() {
            if nome_arquivo == self.arquivos[i].nome {
                removido = true;
                self.arquivos.remove(i);
                break;
            }
        }
        if !removido {
            println!("Arquivo {} não existe dentro do diretório {}", nome_arquivo, self.nome);
        }
    }
}