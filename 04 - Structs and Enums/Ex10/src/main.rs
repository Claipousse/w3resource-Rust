enum OptionValeur<T> {
    Some(T),
    None,
}

fn main() {
    let valeur_some: OptionValeur<i32> = OptionValeur::Some(42);
    let valeur_none: OptionValeur<i32> = OptionValeur::None;

    match valeur_some {
        OptionValeur::Some(val) => println!("valeur : {}", val),
        OptionValeur::None => println!("Aucune valeur"),
    }

    match valeur_none {
        OptionValeur::Some(val) => println!("valeur : {}", val),
        OptionValeur::None => println!("Aucune valeur"),
    }
}