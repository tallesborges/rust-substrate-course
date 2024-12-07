// 5. HashMap com Tuplas como Valores
// Crie um sistema de inventaŕ io para um jogo, onde cada item é mapeado para uma tupla `(quantidade, valor)`. O programa deve permitir que o jogador adicione itens, remova itens e calcule o valor total do inventário.

use std::collections::HashMap;

pub fn main() {
    let mut inventory = Inventory::default();

    // Add some items
    inventory.add_item("Sword".to_string(), 1, 100.0);
    inventory.add_item("Potion".to_string(), 5, 20.0);

    println!("Initial inventory state:");
    println!("Sword: {:?}", inventory.get_item("Sword"));
    println!("Potion: {:?}", inventory.get_item("Potion"));

    // Remove some items
    inventory.remove_item("Potion", 2);
    println!("\nAfter removing 2 potions:");
    println!("Potion: {:?}", inventory.get_item("Potion"));

    // Calculate total
    let total = inventory.calculate_total();
    println!("\nTotal inventory value: {:.2}", total);
}

#[derive(Default)]
struct Inventory {
    items: HashMap<String, (u32, f64)>,
}

impl Inventory {
    fn add_item(&mut self, name: String, quantity: u32, value: f64) {
        self.items.entry(name).or_insert((quantity, value));
    }

    fn remove_item(&mut self, name: &str, quantity: u32) -> bool {
        self.items.get_mut(name).map_or(false, |(q, _)| {
            if *q >= quantity {
                *q -= quantity;
                true
            } else {
                false
            }
        })
    }

    fn calculate_total(&self) -> f64 {
        self.items.iter().fold(0.0, |acc, (_, (quantity, value))| {
            acc + (*quantity as f64) * value
        })
    }

    fn get_item(&self, name: &str) -> Option<&(u32, f64)> {
        self.items.get(name)
    }
}
