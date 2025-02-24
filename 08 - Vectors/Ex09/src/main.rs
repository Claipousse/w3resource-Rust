use std::io;

fn saisie_nombre() -> i32 {
    loop {
        let mut nombre = String::new();
        println!("Saisissez un entier :");
        io::stdin().read_line(&mut nombre).expect("Erreur de lecture");
        match nombre.trim().parse::<i32>() {
            Ok(n) => return n,
            Err(_) => println!("Veuillez saisir un entier valide !")
        }
    }
}

fn presence(vecteur:Vec<i32>, x:i32) {
    if vecteur.contains(&x) {
        println!("{} est bien présent dans le vecteur !", x)
    } else {
        println!("{} n'est pas présent dans le vecteur...", x)
    }
}

fn main() {
    let vecteur:Vec<i32> = (1..=10).collect();
    let nombre:i32 = saisie_nombre();
    presence(vecteur, nombre)
}