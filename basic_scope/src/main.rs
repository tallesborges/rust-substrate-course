fn main() {
    let x = 10;
    {
        let x = 20;
        // will print 20 because x was shadowed
        println!("x = {}", x);
    }

    // will print 10 because 20 lives only in the inner scope
    println!("x = {}", x);
}
