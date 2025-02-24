use std::io;

#[derive(Debug)]
enum Direction {
    Nord,
    Sud,
    Est,
    Ouest
}

fn main() {
    let nord = Direction::Nord;
    let sud = Direction::Sud;
    let est = Direction::Est;
    let ouest = Direction::Ouest;

    println!("Nord: {:?}", nord);
    println!("Sud: {:?}", sud);
    println!("Est: {:?}", est);
    println!("Ouest: {:?}", ouest);
}