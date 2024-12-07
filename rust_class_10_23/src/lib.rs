fn sample_1() {
    let cli = Client {
        name: String::from("John"),
        age: 30,
        active: true,
        balance: 100.0,
    };

    println!("name = {}", cli.name);
    println!("age = {}", cli.age);
    println!("active = {}", cli.active);
    println!("balance = {}", cli.balance);

    let cli2 = create_client(String::from("John"), 30, true, 100.0);
    println!("cli2 = {:#?}", cli2);

    let cli3 = Client {
        name: String::from("Maria"),
        ..cli2
    };

    println!("cli3 = {:?}", cli3);
}

fn sample_2() {
    let location = GLobalLocation(100, 10.0);
    println!("latitude {}, longitude {}", location.0, location.1);
    println!("location = {:?}", location);
}

#[derive(Debug)]
struct GLobalLocation(u64, f64);

#[derive(Debug)]
struct Client {
    name: String,
    age: u8,
    active: bool,
    balance: f64,
}

impl Client {
    fn deposit(&mut self, amount: f64) {
        self.balance += amount;
    }
}

fn create_client(name: String, age: u8, active: bool, balance: f64) -> Client {
    Client {
        name,
        age,
        active,
        balance,
    }
}
