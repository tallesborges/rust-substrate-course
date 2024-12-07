pub fn main() {
    let reference;
    // {
    //     let value = 5;
    //     // reference = &value; not works because value does not live long enough
    // }

    let value = 5;
    reference = &value;
    println!("{}", reference);

    let ref_static: &'static str;
    {
        ref_static = "Hello, world!";
    }
    println!("{}", ref_static)
}
