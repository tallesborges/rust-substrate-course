use examples::{example1, example2, example3};

mod examples;

fn main() {
    example1::main();

    println!("\n{}\n", "=".repeat(15));

    example2::main();

    println!("\n{}\n", "=".repeat(15));

    example3::main();
}
