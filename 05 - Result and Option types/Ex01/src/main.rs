use std::io;

fn saisie() -> String {
    let mut input = String::new();
    println!("Veuillez saisir un nombre :");
    io::stdin().read_line(&mut input).expect("Erreur de lecture...");
    let input = input.trim().to_string();
    input
}

fn conversion_i32(input:String) -> Option<i32> {
    match input.parse::<i32>() {
        Ok(n) => Some(n),
        Err(_) => None
    }
}

fn main() {
    let input:String = saisie();

    match conversion_i32(input) {
        Some(parsed) => println!("Entier parsé : {}", parsed),
        None => println!("Erreur : la conversion a échouée")
    }
}
