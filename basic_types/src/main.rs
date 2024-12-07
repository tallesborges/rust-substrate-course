fn main() {
    let mut age: i32 = 25;
    let weight: f64 = 75.5;
    let active: bool = true;

    println!("My age is {}", age);
    println!("I am {} kilos ", weight);
    println!("and I am {}", if active { "active" } else { "inactive" });

    age = 26;
    println!("My age is {} now", age);
}
