use crate::contract_trader::ContractTrader;
use crate::Action;

pub trait Strategy {
    fn execute(&self, contract: &ContractTrader, saldo: u64, tokens: u64, quotacao: u64) -> Action;
}

#[derive(Clone, Copy)]
#[cfg_attr(
    feature = "std",
    derive(Debug, PartialEq, Eq, ink::storage::traits::StorageLayout)
)]
#[ink::scale_derive(Encode, Decode, TypeInfo)]
pub enum StrategyType {
    Default,
}

impl Strategy for StrategyType {
    fn execute(&self, contract: &ContractTrader, saldo: u64, tokens: u64, quotacao: u64) -> Action {
        match self {
            StrategyType::Default => self.buy_or_sell_plank(contract, saldo, tokens, quotacao),
        }
    }
}

impl StrategyType {
    fn buy_or_sell_plank(
        &self,
        contract: &ContractTrader,
        saldo: u64,
        tokens: u64,
        quotacao: u64,
    ) -> Action {
        let min = ContractTrader::PLANK.checked_div(quotacao).unwrap_or(0);
        let mut action = Action::Idle;

        if saldo > 0 && quotacao < contract.last_quote {
            action = Action::Comprar(min);
        } else if tokens > 0 && quotacao > contract.last_quote {
            action = Action::Vender(min);
        }

        action
    }
}
