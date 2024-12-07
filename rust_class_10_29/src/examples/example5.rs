struct MyString {
    s: String, // s has the ownership
}

struct MyStringRef<'a> {
    s: &'a str, // s does not have the ownership
}

pub fn main() {
    let s = String::from("Hello, world!");
    let string = MyString { s };
    println!("{}", string.s);

    let s = String::from("Hi there!");
    let string_ref = MyStringRef { s: &s };
    println!("{}", string_ref.s);
}
