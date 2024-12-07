fn main() {
    println!("{} Exercise1", "=".repeat(5));
    // exercise 1
    let person1 = Person {
        name: String::from("John"),
        age: 30,
        city: String::from("New York"),
    };

    let person2 = Person {
        name: String::from("Jane"),
        age: 25,
        city: String::from("London"),
    };

    println!("Person 1: {:?}", person1);
    println!("Person 2: {:?}", person2);

    // exercise 2
    println!("{} Exercise2", "=".repeat(5));
    let car = Car {
        brand: String::from("Toyota"),
        model: String::from("Camry"),
        year: 2020,
    };
    println!("Car: {:?}", car);

    let car2 = Car {
        brand: String::from("Honda"),
        ..car
    };

    println!("Car 2: {:?}", car2);

    // exercise 3
    println!("{} Exercise3", "=".repeat(5));
    let coord1 = Coordinate(1.0, 2.0);
    let coord2 = Coordinate(3.0, 4.0);

    println!("Coord 1: {:?}", coord1);
    println!("Coord 2: {:?}", coord2);
    println!("Distance: {}", coord1.distance(&coord2));

    // exercise 4
    println!("{} Exercise4", "=".repeat(5));
    let rect1 = Rect {
        width: 10,
        height: 5,
    };

    println!("Rect 1: {:?}", rect1);
    println!("Area: {}", rect1.area());

    // exercise 5
    println!("{} Exercise5", "=".repeat(5));
    let color = Color::Red;
    match color {
        Color::Red => println!("Color Red"),
        Color::Green => println!("Color Green"),
        Color::Blue => println!("Color Blue"),
    }

    // exercise 6
    println!("{} Exercise6", "=".repeat(5));
    let sum = Operation::Sum(2, 2);
    let division = Operation::Division(2, 2);
    let multiplication = Operation::Multiplication(2, 2);
    let subtraction = Operation::Subtraction(2, 2);

    println!("Sum: {}", sum.result());
    println!("Division: {}", division.result());
    println!("Multiplication: {}", multiplication.result());
    println!("Subtraction: {}", subtraction.result());

    // exercise 7
    println!("{} Exercise7", "=".repeat(5));
    println!("Biggest number: {:?}", biggest_number(&vec![8, 2, 3, 4, 5]));
    println!("Biggest number: {:?}", biggest_number(&vec![]));

    // exercise 8
    println!("{} Exercise8", "=".repeat(5));
    let state = State::On;
    println!("The state is {}", state.description());

    // exercise 9
    println!("{} Exercise9", "=".repeat(5));
    let state2 = State::Off;
    println!("The state is {}", state2.description2());

    // exercise 10
    println!("{} Exercise10", "=".repeat(5));
    let transport = Transport::Bike;
    transport.description();
}

// 1. Criando e Usando Structs Simples
// Crie uma `struct` chamada `Pessoa` que tenha três campos: `nome`, `idade` e `cidade`. Instancie
// duas pessoas diferentes no programa principal e imprima seus dados.
#[derive(Debug)]
struct Person {
    name: String,
    age: u8,
    city: String,
}

// 2. Atualizando Campos de uma Struct
// Crie uma `struct` chamada `Carro` com os campos `marca`, `modelo` e `ano`. Instancie um carro e,
// utilizando a sintaxe de atualização (`update syntax`), crie uma nova instância de `Carro` com
// apenas o campo `ano` alterado.
#[derive(Debug)]
struct Car {
    brand: String,
    model: String,
    year: u16,
}

// 3. Struct com Tuples
// Crie uma `struct tuple` chamada `Coordenada` que tenha dois valores do tipo `f64`. Instancie duas
// coordenadas e calcule a distância entre elas.
#[derive(Debug)]
struct Coordinate(f64, f64);

impl Coordinate {
    fn distance(&self, other: &Coordinate) -> f64 {
        let Coordinate(x0, y0) = self;
        let Coordinate(x1, y1) = other;

        ((x0 - x1).powi(2) + (y0 - y1).powi(2)).sqrt()
    }
}

// 4. Método em Struct
// Crie uma `struct` chamada `Retangulo` que tenha os campos `largura` e `altura`. Implemente um
// método `area` que calcula a área do retângulo. No programa principal, crie um retângulo e imprima
// a área calculada.
#[derive(Debug)]
struct Rect {
    width: u32,
    height: u32,
}

impl Rect {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

// 5. Enumeração Simples
// Crie um `enum` chamado `Cor` que tenha três variantes: `Vermelho`, `Verde`, e `Azul`. No
// programa principal, use um `match` para imprimir uma mensagem diferente para cada cor.
#[derive(Debug)]
enum Color {
    Red,
    Green,
    Blue,
}

// 6. Enum com Parâmetros
// Crie um `enum` chamado `Operacao` que tenha as variantes `Soma(i32, i32)`, `Subtracao(i32,
// i32)`, `Multiplicacao(i32, i32)` e `Divisao(i32, i32)`. Implemente uma função que recebe uma
// `Operacao` e retorna o resultado, usando `match` para realizar a operação correta.
enum Operation {
    Sum(i32, i32),
    Subtraction(i32, i32),
    Multiplication(i32, i32),
    Division(i32, i32),
}

impl Operation {
    fn result(&self) -> i32 {
        match self {
            Operation::Sum(x, y) => x + y,
            Operation::Subtraction(x, y) => x - y,
            Operation::Multiplication(x, y) => x * y,
            Operation::Division(x, y) => x / y,
        }
    }
}

// 7. Enum `Option`
// Crie uma função que recebe uma referência para uma lista de números inteiros e retorna o maior
// número da lista, utilizando `Option<i32>`. Use o `match` para lidar com o caso onde a lista está
// vazia.
fn biggest_number(numbers: &[i32]) -> Option<i32> {
    if numbers.is_empty() {
        return None;
    }

    let mut biggest = numbers[0];
    for &n in numbers.iter() {
        biggest = n;
    }

    Some(biggest)

    // numbers.iter().max().copied()
}

// 8. Funções Associadas em Enums
// Crie um `enum` chamado `Estado` com as variantes `Ligado` e `Desligado`. Implemente uma
// função associada ao `enum` que retorna a descrição textual do estado atual. Teste a função no
// programa principal.
enum State {
    On,
    Off,
}

impl State {
    fn description(&self) -> String {
        match self {
            State::On => "On".to_owned(),
            State::Off => "Off".to_owned(),
        }
    }
}

// 9. Uso de `if let`
// Modifique o exercício anterior para utilizar `if let` em vez de `match` ao verificar se o estado é
// `Ligado` antes de imprimir uma mensagem.
impl State {
    fn description2(&self) -> String {
        if let State::On = self {
            "On".to_owned()
        } else {
            "Off".to_owned()
        }
    }
}

// 10. Match com Múltiplos Comandos
// Crie um `enum` chamado `Transporte` com as variantes `Carro`, `Bicicleta`, `Caminhada`. Para
// cada variante, use um `match` que imprime múltiplas mensagens, como a velocidade média e o
// impacto ambiental do meio de transporte escolhido.
enum Transport {
    Car,
    Bike,
    Walk,
}

impl Transport {
    fn description(&self) {
        let result = match self {
            Transport::Car => (100, 100),
            Transport::Bike => (20, 50),
            Transport::Walk => (5, 0),
        };

        println!("Average speed: {} km/h", result.0);
        println!("Impact on the environment: {}%", result.1);

        // match self {
        //     Transport::Car => {
        //         println!("Average speed: 100 km/h");
        //         println!("Impact on the environment: 100%");
        //     }
        //     Transport::Bike => {
        //         println!("Average speed: 20 km/h");
        //         println!("Impact on the environment: 50%");
        //     }
        //     Transport::Walk => {
        //         println!("Average speed: 5 km/h");
        //         println!("Impact on the environment: 0%");
        //     }
        // }
    }
}
