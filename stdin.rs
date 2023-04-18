use std::io;

fn main() {
  let mut input = String::new();
  println!("Digite um texto: ");
  
  io::stdin()
    .read_line(&mut input)
    .expect("Erro ao ler o texto");
    
  println!("Você digitou: {}", input);
}
