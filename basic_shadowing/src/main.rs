fn main() {
    shadowing();
    using_mut();

    // both will print 16
}

fn shadowing() {
    let num = 5;

    let num = num + 3;

    let num = num * 2;

    println!("num = {}", num);
}

fn using_mut() {
    let mut num = 5;

    num = num + 3;

    num = num * 2;

    println!("num = {}", num);
}
