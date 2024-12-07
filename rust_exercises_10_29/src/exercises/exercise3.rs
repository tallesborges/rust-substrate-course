// 3. Função com Lifetimes Diferentes
// Implemente uma função concat_with_prefix que recebe duas referências de
// lifetime diferentes: uma &str com prefixo e uma &str para o conteúdo. Retorne
// uma nova String que concatene o prefixo e o conteúdo, garantindo que os
// lifetimes sejam respeitados.

pub fn concat_with_prefix<'a, 'b>(prefix: &'a str, content: &'b str) -> String {
    String::from(prefix) + content
}

pub fn main() {
    let prefix = "Hello, ";
    let content = "World!";
    let result = concat_with_prefix(prefix, content);
    println!("{}", result);
}
