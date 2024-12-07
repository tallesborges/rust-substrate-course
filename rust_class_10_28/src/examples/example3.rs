#[derive(Debug, Copy, Clone)]
struct Fahrenheit(f64);

#[derive(Debug, Copy, Clone)]
struct Celsius(f64);

impl From<Fahrenheit> for Celsius {
    fn from(f: Fahrenheit) -> Celsius {
        Celsius((f.0 - 32.0) * 1.8)
    }
}

impl From<Celsius> for Fahrenheit {
    fn from(c: Celsius) -> Fahrenheit {
        Fahrenheit(c.0 * 1.8 + 32.0)
    }
}

enum Temperature {
    Celsius(f64),
    Farenheit(f64),
}

pub fn main() {
    let temp_f = Fahrenheit(32.0);
    let temp_c = Celsius::from(temp_f);
    println!("{temp_f:?} to {temp_c:?}");

    let temp_c = Celsius(0.0);
    let temp_f: Fahrenheit = temp_c.into();
    println!("{temp_f:?} to {temp_c:?}");
}
