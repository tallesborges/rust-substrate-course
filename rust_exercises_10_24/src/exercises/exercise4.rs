use std::collections::HashMap;

// 4. Diferenca̧ entre HashMaps
// Implemente uma funçaõ que receba dois `HashMap<String, u32>` e retorne um novo `HashMap<String, u32>` contendo apenas os pares chave-valor presentes no primeiro `HashMap`, mas não no segundo.
pub fn main() {
    let mut map1 = HashMap::new();
    map1.insert(String::from("a"), 1);
    map1.insert(String::from("b"), 2);
    map1.insert(String::from("c"), 3);

    let mut map2 = HashMap::new();
    map2.insert(String::from("a"), 1);
    map2.insert(String::from("b"), 5);

    let result = find_difference(map1, map2);
    println!("{:?}", result);
}

fn find_difference(map1: HashMap<String, u32>, map2: HashMap<String, u32>) -> HashMap<String, u32> {
    let mut result = HashMap::new();

    for (key, value) in map1.iter() {
        if !map2.contains_key(key) {
            result.insert(key.clone(), *value);
        }
    }

    result
}
