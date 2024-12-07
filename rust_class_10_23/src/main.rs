mod sample2;
mod sample4;

pub fn main() {
    let mut account = sample2::Account { balance: 100.0 };
    println!("Account balance: ${}", account.read_balance());

    println!("Transferring $50...");
    account.transfer(50.0);
    println!("Account balance: ${}", account.read_balance());

    let balance = account.close();
    println!("Final balance: ${}", balance);

    // sample 4
    println!("==== sample 4 ====");
    println!("Division result: {:?}", sample4::divide(10, 2));
    let result = sample4::divide(10, 0);
    match result {
        Some(x) => println!("Some result: {}", x),
        None => println!("None result"),
    }

    let numbers = vec![1, 2, 3, 4, 5];
    let result = sample4::find_number(3, &numbers);
    println!("Result: {:?}", result);

    // sample 5
    println!("==== sample 5 ====");
    let x = 5;
    let y = Some(5);
    // let result = x + y; // doesnt work because of type mismatch
    let result = match y {
        Some(z) => x + z,
        None => x,
    };
    println!("Result: {}", result);

    if let Some(z) = y {
        println!("Some: {}", z);
    } else {
        println!("None");
    }
}
