use std::collections::HashSet;
use std::io;
use rand::Rng;

fn saisie_taille() -> usize {
    loop {
        let mut taille = String::new();
        println!("Combien d'entiers voulez-vous dans le vecteur ?");
        io::stdin().read_line(&mut taille).expect("Erreur de lecture...");
        match taille.trim().parse::<usize>() {
            Ok(n) => return n,
            Err(_) => println!("Erreur : Veuillez saisir une taille correcte.")
        }
    }
}

fn vecteur_to_set(vecteur:&Vec<i32>) -> HashSet<i32>{
    let mut set:HashSet<i32> = HashSet::new();
    for &element in vecteur {
        set.insert(element);
    }
    set
}

fn main() {
    let mut rng = rand::rng();
    let taille:usize = saisie_taille();
    let vecteur:Vec<i32> = (0..taille).map(|_| rng.random_range(0..=100)).collect();
    let set:HashSet<i32> = vecteur_to_set(&vecteur);
    println!("{:?} -> {:?}", vecteur, set)
}