fn main() {
    let mut s = String::from("Rust");

    let r1 = &s;
    let r2 = &s;
    let r3 = &s;

    println!("{}, {}, {}", r1, r2, r3);

    let r4 = &mut s;
    println!("{}", r4);

    r4.push_str(" is pretty cool");
    println!("{}", r4);

    let r5 = &mut s;
    println!("{}", r5);

    println!("{}", r4);
}
