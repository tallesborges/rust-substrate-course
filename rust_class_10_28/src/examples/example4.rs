use core::f64;
use std::default;

#[derive(PartialEq, PartialOrd)]
struct Product {
    // name: String,
    price: f64,
}

#[derive(Default)]
struct Book {
    title: String,
    author: String,
    pages: usize,
}

pub fn main() {
    let product1 = Product {
        // name: String::from("Apple"),
        price: 0.99,
    };
    let product2 = Product {
        // name: String::from("Orange"),
        price: 1.49,
    };

    println!("{:?}", product1 > product2);

    // partialEq vs Eq

    let x = 2.7;
    let y = f64::NAN;

    println!("{}", x == x);
    println!("{}", y == y);

    let book1 = Book {
        title: String::from("The Great Gatsby"),
        author: String::from("F. Scott Fitzgerald"),
        pages: 180,
    };

    let book2 = Book::default();
}
