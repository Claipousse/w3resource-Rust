use std::io;

fn saisie_lettre() -> char {
    loop {
        let mut lettre = String::new();
        println!("Saisissez la première lettre de votre prénom :");
        io::stdin().read_line(&mut lettre).expect("Erreur de lecture...");
        match lettre.trim().parse::<char>() {
            Ok(n) => return n,
            Err(_) => println!("La saisie est incorrecte, Veuillez réessayer.")
        }
    }
}

fn affichage(lettre:char) {
    println!("La première lettre de votre prénom est : '{}'", lettre)
}

fn main() {
    let lettre:char = saisie_lettre();
    affichage(lettre)
}