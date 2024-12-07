use std::fmt::Display;

trait BlockchainItem {
    fn id(&self) -> usize;
}

struct Block<'a, T>
where
    T: BlockchainItem + Display,
{
    data: &'a T,
}

impl<'a, T> Block<'a, T>
where
    T: BlockchainItem + Display,
{
    fn show_id(&self) {
        println!("Block ID: {}", self.data.id());
    }

    fn show_data(&self) {
        println!("Block data: {}", self.data);
    }
}

struct Transaction {
    id: usize,
    amount: u32,
}

impl BlockchainItem for Transaction {
    fn id(&self) -> usize {
        self.id
    }
}

impl Display for Transaction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Transaction id: {}, amount: {}", self.id, self.amount)
    }
}

pub fn main() {
    let transaction = Transaction { id: 1, amount: 100 };
    let block = Block { data: &transaction };

    block.show_id();
    block.show_data();
}
