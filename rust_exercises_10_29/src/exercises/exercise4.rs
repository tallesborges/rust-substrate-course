// 4. Struct com Lifetimes Aninhados
// Implemente uma estrutura Context<'a, 'b> que contenha referências para duas
// outras estruturas, A e B. Ambas as estruturas devem conter lifetimes diferentes ('a
// e 'b). Implemente um método em Context que modifique uma das referências com
// base no valor da outra, utilizando lifetimes corretamente.

struct FirstStruct {
    value: usize,
}
struct SecondStruct {
    value: usize,
}

struct DataContext<'a, 'b> {
    first: &'a mut FirstStruct,
    second: &'b SecondStruct,
}

impl<'a, 'b> DataContext<'a, 'b> {
    fn update_data(&mut self) -> bool {
        self.first.value += self.second.value;
        true
    }
}

pub fn main() {
    let mut first = FirstStruct { value: 5 };
    let second = SecondStruct { value: 10 };
    let mut context = DataContext {
        first: &mut first,
        second: &second,
    };
    context.update_data();
    println!("Updated value: {}", first.value);
}
