struct Pessoa {
    primeiro_nome: String,
    ultimo_nome: String,
}

impl Pessoa {
    fn nome_completo(&self) -> String {
        format!("{} {}", self.primeiro_nome, self.ultimo_nome)
    }
}

fn main() {
    let pessoa = Pessoa {
        primeiro_nome: String::from("João"),
        ultimo_nome: String::from("Silva"),
    };

    println!("O nome completo da pessoa é: {}", pessoa.nome_completo());
}
