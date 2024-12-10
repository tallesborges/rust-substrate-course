#![cfg_attr(not(feature = "std"), no_std, no_main)]

mod strategies;

// traits definition
#[derive(Clone, Copy)]
#[cfg_attr(
    feature = "std",
    derive(Debug, PartialEq, Eq, ink::storage::traits::StorageLayout)
)]
#[ink::scale_derive(Encode, Decode, TypeInfo)]
pub enum Action {
    Idle,
    Comprar(u64),
    Vender(u64),
}

#[ink::trait_definition]
pub trait Trader {
    #[ink(message)]
    fn setup(&mut self, saldo: u64, tokens: u64);

    #[ink(message)]
    fn tick(&mut self, timestamp: u64, saldo: u64, tokens: u64, quotacao: u64) -> Action;
}

#[ink::contract]
mod contract_trader {
    use crate::strategies::*;

    use super::Action;
    use super::Trader;

    #[ink(storage)]
    pub struct ContractTrader {
        pub strategy: StrategyType,
        pub fiat_balance: u64,
        pub token_balance: u64,
        pub last_quote: u64,
        pub last_timestamp: u64,
    }

    impl ContractTrader {
        pub const PLANK: u64 = 1_000_000_000_000;

        #[ink(constructor)]
        pub fn new() -> Self {
            Self {
                strategy: StrategyType::Default,
                fiat_balance: 0,
                token_balance: 0,
                last_quote: 0,
                last_timestamp: 0,
            }
        }

        #[ink(message)]
        pub fn get_balance(&self) -> (u64, u64) {
            (self.fiat_balance, self.token_balance)
        }

        #[ink(message)]
        pub fn get_last_quote(&self) -> u64 {
            self.last_quote
        }

        fn update_balances(&mut self, action: Action, quotacao: u64) {
            match action {
                Action::Vender(amount) => {
                    self.token_balance = self.token_balance.checked_sub(amount).unwrap_or(0);
                    let quote_mul = quotacao.checked_mul(amount).unwrap_or(0);
                    let final_amount = quote_mul.checked_div(Self::PLANK).unwrap_or(1);

                    self.fiat_balance = self
                        .fiat_balance
                        .checked_add(final_amount)
                        .unwrap_or(self.fiat_balance);
                }
                Action::Comprar(amount) => {
                    let quote_mul = quotacao.checked_mul(amount).unwrap_or(0);
                    let cost = quote_mul.checked_div(Self::PLANK).unwrap_or(0);

                    self.fiat_balance = self
                        .fiat_balance
                        .checked_sub(cost.max(1))
                        .unwrap_or(self.fiat_balance);
                    self.token_balance = self
                        .token_balance
                        .checked_add(amount)
                        .unwrap_or(self.token_balance);
                }
                Action::Idle => (),
            }
        }
    }

    impl Trader for ContractTrader {
        #[ink(message)]
        fn setup(&mut self, saldo: u64, tokens: u64) {
            self.fiat_balance = saldo;
            self.token_balance = tokens;
        }

        #[ink(message)]
        fn tick(&mut self, timestamp: u64, saldo: u64, tokens: u64, quotacao: u64) -> Action {
            self.fiat_balance = saldo;
            self.token_balance = tokens;

            if self.last_timestamp == 0 {
                self.last_quote = quotacao;
                self.last_timestamp = timestamp;
                return Action::Idle;
            }

            let action = self.strategy.execute(self, saldo, tokens, quotacao);
            self.update_balances(action, quotacao);

            self.last_quote = quotacao;
            self.last_timestamp = timestamp;

            action
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[ink::test]
        fn setup_works() {
            let mut contract = ContractTrader::new();
            contract.setup(5010, 1_000_000_000_000);
            assert_eq!(contract.fiat_balance, 5010);
            assert_eq!(contract.token_balance, 1_000_000_000_000);
        }

        #[ink::test]
        fn tick_works() {
            let mut contract = ContractTrader::new();
            let initial_fiat = 10000;
            let initial_balance = 1_000_000_000_000;
            contract.setup(5010, initial_balance);

            // the first tick alawys is idle
            let action = contract.tick(1, initial_fiat, initial_balance, 0);
            assert_eq!(action, Action::Idle);

            // price goes up
            let action = contract.tick(2, initial_fiat, initial_balance, 500);
            assert_eq!(action, Action::Vender(2_000_000_000));
            assert_eq!(contract.get_balance(), (10001, 998_000_000_000));

            // price goes down
            let action = contract.tick(3, initial_fiat, initial_balance, 400);
            assert_eq!(action, Action::Comprar(2_500_000_000));
            assert_eq!(contract.get_balance(), (9999, 1_002_500_000_000));

            // price goes up
            let action = contract.tick(4, initial_fiat, initial_balance, 1000);
            assert_eq!(action, Action::Vender(1000000000));
            assert_eq!(contract.get_balance(), (10001, 999000000000));

            // price goes down
            let action = contract.tick(5, initial_fiat, initial_balance, 883);
            assert_eq!(action, Action::Comprar(1132502831));
            assert_eq!(contract.get_balance(), (9999, 1001132502831));
        }

        #[ink::test]
        fn read_csv_prices() {
            let mut contract = ContractTrader::new();
            contract.setup(10000, 1_000_000_000_000);

            // Read CSV file
            let csv_content = include_str!("polkadot_prices.csv");

            for line in csv_content.lines() {
                if let Some((timestamp_str, price_str)) = line.split_once(',') {
                    let timestamp = timestamp_str.parse::<u64>().unwrap();
                    let price = (price_str.parse::<u64>().unwrap()) as u64;

                    let balance = contract.get_balance();
                    let action = contract.tick(timestamp, balance.0, balance.1, price);

                    match action {
                        Action::Comprar(amount) => {
                            println!("Buy {} at price {}", amount, price);
                        }
                        Action::Vender(amount) => {
                            println!("Sell {} at price {}", amount, price);
                        }
                        Action::Idle => println!("Idle at timestamp {}", timestamp),
                    }
                    println!("Final balance: {:?}", contract.get_balance());
                }
            }

            // print final balance
            let balance = contract.get_balance();
            println!("Final balance: {:?}", balance);
        }
    }
}
