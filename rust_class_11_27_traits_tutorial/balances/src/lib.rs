use std::collections::HashMap;

pub type AccountId = u32;
pub type Balance = u32;

#[derive(Default)]
pub struct BalancesModule {
    balances: HashMap<AccountId, Balance>,
}

#[derive(Debug, PartialEq, Eq, thiserror::Error)]
pub enum Error {
    #[error("InsufficientBalance")]
    InsufficientBalance,
    #[error("Overflow")]
    Overflow,
}

impl BalancesModule {
    pub fn mint(&mut self, account: AccountId, amount: Balance) {
        let old_balance = self.balances.get(&account);
        let new_balance = old_balance.unwrap_or(&0) + amount;
        self.balances.insert(account, new_balance);
    }

    pub fn balances(&self, account: AccountId) -> Balance {
        self.balances.get(&account).copied().unwrap_or_default()
    }

    pub fn tranfer(
        &mut self,
        from: AccountId,
        to: AccountId,
        amount: Balance,
    ) -> Result<(), Error> {
        let Some(balance) = self.balances.get_mut(&from) else {
            return Err(Error::InsufficientBalance);
        };

        let old_balance = self.balances.get(&from).copied().unwrap_or_default();

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mint_works() {
        let mut balances = BalancesModule::default();
        balances.mint(1, 10);
        balances.mint(1, 20);
        balances.mint(2, 30);
        balances.mint(2, 40);
        assert_eq!(balances.balances(1), 30);
        assert_eq!(balances.balances(2), 70);
    }
}
