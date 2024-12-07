// 2. Contagem de Caracteres Repetidos com Unicode
// Modifique o exercício de contagem de ocorren̂ cias de palavras para contar a frequência de caracteres em uma string que contém caracteres Unicode. Utilize um `HashMap<char, u32>` para armazenar as frequen̂ cias.

use std::collections::HashMap;

pub fn main() {
    let world = String::from("Hello World!");

    let char_counts = count_chars(&world);
    println!("Character counts: {:?}", char_counts);
}

pub fn count_chars(text: &str) -> HashMap<char, u32> {
    let mut char_counts = HashMap::new();

    for c in text.chars() {
        *char_counts.entry(c).or_insert(0) += 1;
    }

    char_counts
}
