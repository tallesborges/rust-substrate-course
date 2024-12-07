use balances::BalancesModule;

fn main() {
    let mut balances = BalancesModule::default();
    balances.mint(1, 10);
    balances.mint(1, 20);
    balances.mint(2, 30);
    balances.mint(2, 40);
    println!("balances: {:?}", balances.balances(1));
}
