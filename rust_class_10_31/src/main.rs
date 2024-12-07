fn add(a: u32, b: u32) -> u32 {
    a + b
}

#[cfg(feature = "my_feature")]
fn multiply(a: u32, b: u32) -> u32 {
    a * b
}

fn main() {
    if cfg!(feature = "my_feature") {
        println!("1 * 2 = {}", multiply(1, 2));
    } else {
        let x = add(1, 2);
        println!("1 + 2 = {}", x);
    }
}

#[cfg(test)]
mod tests {
    use super::add;

    #[test]
    fn test_add() {
        assert_eq!(add(1, 2), 3);
    }
}
