use std::io;
use rand::Rng;

fn saisie_taille() -> usize {
    loop {
        let mut taille = String::new();
        println!("Veuillez saisir la taille du vecteur :");
        io::stdin().read_line(&mut taille).expect("Erreur de lecture...");
        match taille.trim().parse::<usize>() {
            Ok(n) => return n,
            Err(_) => println!("Veuillez saisir une taille correcte !")
        }
    }
}

fn maximum(vec:&Vec<i32>) -> Option<i32> {
    vec.iter().max().cloned()
}

fn main() {
    let mut rng = rand::rng();
    let taille:usize = saisie_taille();
    let vecteur:Vec<i32> = (0..taille).map(|_| rng.random_range(0..=1000)).collect();

    match maximum(&vecteur) {
        Some(max) => {
            println!("{:?}", vecteur);
            println!("Le maximum du vecteur est : {}", max)
        }
        None => println!("Erreur : Le vecteur est vide.")
    }
}