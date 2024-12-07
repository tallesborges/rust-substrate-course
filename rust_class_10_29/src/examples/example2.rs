fn return_ref(str: &str) -> &str {
    str
}

fn return_ref2<'a>(str: &'a str) -> &'a str {
    str
}

fn bigger<'a>(str1: &'a str, str2: &'a str) -> &'a str {
    if str1.len() > str2.len() {
        str1
    } else {
        str2
    }
}

struct Book {
    // title: &str;
    title: String,
}

pub fn main() {
    let str = String::from("Hello, world!");
    let reference = return_ref(&str);

    println!("{}", reference);

    let reference2 = return_ref2(&str);
    println!("{}", reference2);

    let first = String::from("Hello");
    let second = String::from("Hello, world!");
    let bigger_str = bigger(&first, &second);
    println!("{}", bigger_str);
}
