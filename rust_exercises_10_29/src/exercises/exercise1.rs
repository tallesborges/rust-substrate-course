// 1. Lifetime em Struct
// Defina uma estrutura Point<'a> que contenha duas referências a i32. Implemente
// uma função que, dado um Point, retorne o maior valor entre os dois.

struct Point<'a> {
    x: &'a i32,
    y: &'a i32,
}

fn find_largest<'a>(point: &'a Point) -> &'a i32 {
    if point.x > point.y {
        point.x
    } else {
        point.y
    }
}

pub fn main() {
    let a = 10;
    let b = 5;
    let point = Point { x: &a, y: &b };
    println!(
        "Largest value between {} and {} is: {}",
        a,
        b,
        find_largest(&point)
    );

    let c = 3;
    let d = 8;
    let point2 = Point { x: &c, y: &d };
    println!(
        "Largest value between {} and {} is: {}",
        c,
        d,
        find_largest(&point2)
    );

    let e = 15;
    let f = 15;
    let point3 = Point { x: &e, y: &f };
    println!(
        "Largest value between {} and {} is: {}",
        e,
        f,
        find_largest(&point3)
    );
}
