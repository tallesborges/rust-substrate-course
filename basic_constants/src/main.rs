const PI: f64 = 3.1415;
const GRAVITY: f32 = 9.8;

fn main() {
    let radious = 10.0;

    let area = PI * radious * radious;

    PI = 3;

    println!("area = {}", area);
}
