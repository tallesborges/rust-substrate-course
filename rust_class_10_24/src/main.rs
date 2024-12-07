use std::{
    collections::HashMap,
    fs::File,
    io::{self, ErrorKind, Read},
};

fn main() {
    // sample 1
    let mut movies = HashMap::new();
    movies.insert("The Shawshank Redemption", 1994);
    movies.insert("The Godfather", 1972);
    movies.insert("The Godfather: Part II", 1974);
    movies.insert("The Dark Knight", 2008);
    movies.insert("12 Angry Men", 1957);
    movies.insert("Schindler's List", 1993);
    movies.insert("1997", 8);

    for (title, year) in movies.iter() {
        println!("{}: {}", title, year);
    }

    let movie = movies.get("12 Angry Men");
    println!("{:?}", movie);

    let title = String::from("Her");
    let year = 2010;
    movies.insert(&title, year);
    println!("{:?}", movies);

    movies.entry("1997").or_insert(10);
    println!("{:?}", movies);

    // sample 2
    let result = divide(2, 0);
    println!("Result {:?}", result);

    let result = match divide(2, 0) {
        Ok(v) => v,
        Err(e) => {
            println!("{}", e);
            0
        }
    };
    println!("Result {}", result);
}

// sample 2
fn divide(x: i32, y: i32) -> Result<i32, String> {
    if y == 0 {
        return Err("Cannot divide by zero".to_string());
    }

    Ok(x / y)
}

// sample 3
fn open_file() -> File {
    let file = File::open("file.txt");

    let file = match file {
        Ok(f) => f,
        Err(e) => match e.kind() {
            ErrorKind::NotFound => match File::create("file.txt") {
                Err(e) => panic!("Error creating file: {}", e),
                Ok(f) => f,
            },
            _ => panic!("Unexpected error"),
        },
    };

    file
}

fn read_file() -> Result<String, io::Error> {
    let mut s = String::new();
    let mut f = File::open("file.txt")?;

    f.read_to_string(&mut s)?;

    Ok(s)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_divide() {
        assert_eq!(divide(10, 2), Ok(5));
    }
}
