use class3::calculate_salary;
use rand::Rng;

fn main() {
    print_salary();
    print_for_loop();
    print_string_heap();
    print_string_stack();
}

fn print_salary() {
    let mut rng = rand::thread_rng();
    let base_salary = rng.gen_range(1..101);
    let salary = calculate_salary(base_salary, 20);
    println!("The salary is: {}", salary);
}

fn print_for_loop() {
    for i in 0..10 {
        println!("i: {}", i);
    }
}

fn print_string_heap() -> String {
    let mut s = String::from("Hello");
    s.push_str(", world!");
    println!("{}", s);

    s
}

fn print_string_stack() -> &'static str {
    let s = "Hello, world!";
    println!("{}", s);

    s
}
