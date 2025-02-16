enum Item {
    Entier(i32),
    Float(f64),
    Texte(String),
}

fn main() {
    let items = vec![
        Item::Entier(42),
        Item::Float(3.14),
        Item::Texte(String::from("Ceci est un texte!")),
    ];

    for item in &items {
        match item {
            Item::Entier(i) => println!("{}", i),
            Item::Float(f) => println!("{}", f),
            Item::Texte(s) => println!("{}", s),
        }
    }
}