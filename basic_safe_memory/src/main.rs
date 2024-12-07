fn main() {
    let data: String = String::from("Safe memory!");

    print_data(data);

    println!("data: {:?}", data);
}

fn print_data(data: String) {
    println!("data: {}", data);
}
