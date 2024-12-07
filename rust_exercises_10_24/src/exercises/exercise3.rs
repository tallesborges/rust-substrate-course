// 3. Colapso de HashMap com Chaves Compostas
// Dado um `HashMap<(String, String), u32>`, onde a primeira parte da chave é uma categoria e a segunda é um item, crie uma funçaõ que retorne um novo `HashMap<String, u32>` colapsando todas as categorias e somando os valores de seus itens correspondentes.

use std::collections::HashMap;

pub fn main() {
    let foods = HashMap::from([
        ((String::from("Food"), String::from("Apple")), 10),
        ((String::from("Food"), String::from("Orange")), 14),
        ((String::from("Food"), String::from("Banana")), 8),
        ((String::from("Drink"), String::from("Water")), 5),
        ((String::from("Drink"), String::from("Juice")), 7),
        ((String::from("Snack"), String::from("Chips")), 12),
        ((String::from("Snack"), String::from("Nuts")), 9),
    ]);

    let collapsed = collapse_categories(foods);
    println!("{:?}", collapsed);
}

fn collapse_categories(map: HashMap<(String, String), u32>) -> HashMap<String, u32> {
    let mut output = HashMap::new();

    for ((category, _item), value) in map.iter() {
        *output.entry(category.to_string()).or_insert(0) += value;
    }

    output
}
