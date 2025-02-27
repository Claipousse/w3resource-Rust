use std::io;

fn saisie_nombre() -> i32 {
    loop {
        let mut n = String::new();
        println!("Veuillez saisir un entier :");
        io::stdin().read_line(&mut n).expect("Erreur de lecture...");
        match n.trim().parse::<i32>() {
            Ok(n) => return n,
            Err(_) => println!("Veuillez saisir un entier correct !")
        }
    }
}

fn carre(nombre:i32) -> Option<i32> {
    if nombre >= 0 {
        Some(nombre * nombre)
    } else {
        None
    }
}

fn main() {
    let nombre:i32 = saisie_nombre();

    match carre(nombre) {
        Some(n) => println!("Le carré de {} est : {}", nombre, n),
        None => println!("Erreur : Le nombre saisit est négatif")
    }
}