use std::io;
use rand::Rng;
fn lire_entier(message:&str) -> i32 {
    loop {
        let mut input = String::new();
        println!("{}", message);
        io::stdin().read_line(&mut input).expect("Erreur de lecture");
        match input.trim().parse::<i32>() {
            Ok(n) => return n,
            Err(_) => println!("Saisissez un entier !")
        }
    }
}

fn saisie_tuple() -> (i32, i32) {
    let n1:i32 = lire_entier("Saisissez le 1er entier :");
    let n2:i32 = lire_entier("Saisissez le 2eme entier :");
    (n1, n2)
}

fn affichage(tuple:(i32, i32)) {
    let mut rng = rand::rng();
    let index = rng.random_range(0..2);
    if index == 0 {
        println!("La première valeur du tuple est : {}",tuple.0)
    }
    else {
        println!("La deuxième valeur du tuple est : {}",tuple.1)
    }
}

fn main() {
    let tuple:(i32, i32) = saisie_tuple();
    affichage(tuple);
}