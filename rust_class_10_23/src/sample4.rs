pub fn divide(x: i32, y: i32) -> Option<i32> {
    if y == 0 {
        return None;
    }

    Some(x / y)
}

pub fn find_number(x: i32, numbers: &[i32]) -> Option<i32> {
    for &n in numbers {
        if n == x {
            return Some(n);
        }
    }

    Some(x)
}
