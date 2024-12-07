// 1. Mapeamento de Parentesco
// Crie uma estrutura que modele um sistema de árvore genealógica. Utilize um `HashMap<String, Vec<String>>` para representar a relaçaõ entre pais e filhos, onde cada chave é o nome de uma pessoa e o valor é uma lista dos nomes de seus filhos. Implemente uma funca̧ ̃o que, dada uma pessoa, retorne todos os seus descendentes diretos e indiretos.

use std::collections::HashMap;

pub fn get_descendants(name: &str, family_tree: &HashMap<String, Vec<String>>) -> Vec<String> {
    let mut descendants = Vec::new();

    if let Some(children) = family_tree.get(name) {
        for child in children {
            descendants.push(child.clone());
            descendants.extend(get_descendants(child, family_tree));
        }
    }

    descendants
}

pub fn create_family_tree() -> HashMap<String, Vec<String>> {
    let mut family_tree = HashMap::new();

    family_tree.insert(
        "John".to_string(),
        vec!["Mary".to_string(), "Peter".to_string()],
    );
    family_tree.insert(
        "Mary".to_string(),
        vec!["Tom".to_string(), "Ann".to_string()],
    );
    family_tree.insert("Peter".to_string(), vec!["Sarah".to_string()]);

    family_tree
}

pub fn main() {
    let family_tree = create_family_tree();

    println!("Family Tree Structure:");
    for (parent, children) in &family_tree {
        println!("{} -> {:?}", parent, children);
    }

    let root = "John";
    let descendants = get_descendants(root, &family_tree);
    println!("\nAll descendants of {}:", root);
    println!("{:?}", descendants);
}
