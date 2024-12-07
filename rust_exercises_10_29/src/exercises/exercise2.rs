// 2. Retorno de Referências em Structs
// Crie uma estrutura Owner<'a> que contenha uma referência a uma String.
// Implemente um método que retorne uma fatia da String que a estrutura contém,
// utilizando lifetimes apropriados.

struct Owner<'a> {
    data: &'a String,
}

impl<'a> Owner<'a> {
    fn get_slice(&self, start: usize, end: usize) -> &str {
        return &self.data[start..end];
    }
}

pub fn main() {
    let s = String::from("Hello, world!");
    let owner = Owner { data: &s };
    println!("Slice: {}", owner.get_slice(7, 12));
}
