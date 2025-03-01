use std::io;
use rand::Rng;

fn saisie_taille() -> usize {
    loop {
        let mut taille = String::new();
        println!("Combien d'entiers voulez-vous dans le vecteur ?");
        io::stdin().read_line(&mut taille).expect("Erreur de lecture...");
        match taille.trim().parse::<usize>() {
            Ok(n) => return n,
            Err(_) => println!("Erreur : format incorrect")
        }
    }
}

fn tri_impair(vecteur: &Vec<i32>) -> Option<Vec<i32>> {
    if vecteur.is_empty() {
        None
    } else {
        Some(vecteur.iter().copied().filter(|x| x % 2 != 0).collect())
    }
}

fn main() {
    let mut rng = rand::rng();
    let vecteur:Vec<i32> = (0..saisie_taille()).map(|_| rng.random_range(0..=100)).collect();
    match tri_impair(&vecteur) {
        Some(vec_impair) => {
            println!("\nVecteur : {:?}", vecteur);
            if vec_impair.is_empty() {
                println!("Il n'y a aucun nombre impair dans le vecteur.")
            } else {
                println!("Vecteur (impair) : {:?}", vec_impair)
            }
        }
        None => println!("Erreur : Le vecteur est vide...")
    }
}