fn main() {
    let num1: i32 = 5;
    let num2: i32 = 10;
    println!("The sum of 1 and 2 is {}", sum(num1, num2));
}

fn sum(a: i32, b: i32) -> i64 {
    a + b
}
