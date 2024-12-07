fn main() {
    println!("safe_division(10, 2): {:?}", safe_division(10, 2));
    println!("safe_division(10, 0): {:?}", safe_division(10, 0));
}

fn safe_division(a: i32, b: i32) -> Option<i32> {
    if b == 0 {
        None
    } else {
        Some(a / b)
    }
}
