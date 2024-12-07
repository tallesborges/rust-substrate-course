use std::fmt::Debug;

fn combine<T, U>(a: T, b: U) -> (T, U) {
    (a, b)
}

fn duplicate<T>(x: T) -> (T, T)
where
    T: Clone + Debug,
{
    println!("x: {:?}", x);
    (x.clone(), x)
}

pub fn main() {
    let tuple: (i32, i32) = combine(1, 2);
    println!("({}, {})", tuple.0, tuple.1);

    let other_tuple = duplicate("hello");
    println!("({}, {})", other_tuple.0, other_tuple.1);
}
