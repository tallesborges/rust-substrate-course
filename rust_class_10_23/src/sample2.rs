#[derive(Debug)]
pub struct Account {
    pub balance: f32,
}

impl Account {
    pub fn read_balance(&self) -> f32 {
        self.balance
    }
}

impl Account {
    pub fn transfer(&mut self, amount: f32) {
        if self.balance >= amount {
            self.balance -= amount;
        }
    }
}

impl Account {
    pub fn close(self) -> f32 {
        println!("Closing account with balance: ${}", self.balance);

        self.balance
    }
}
