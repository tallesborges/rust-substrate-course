struct Book<'a> {
    title: &'a str,
}

pub fn main() {
    let title = String::from("The Adventures");
    let book = Book { title: &title };
    println!("Title: {}", book.title);
    println!("Title Orignal: {}", title);
}
