use std::io;
use rand::Rng;

fn saisie_taille() -> usize {
    loop {
        let mut taille = String::new();
        println!("Combien d'entiers voulez-vous dans le vecteur ? (Max 20)");
        io::stdin().read_line(&mut taille).expect("Erreur de lecture...");
        match taille.trim().parse::<usize>() {
            Ok(n) if n <= 20 => return n,
            _ => println!("Veuillez saisir une taille correcte !")
        }
    }
}

fn affichage(v:Vec<i32>, v_impair:Vec<i32>, sub_v:Vec<i32>) {
    println!("\nVecteur : {:?}", v);
    println!("Vecteur impair : {:?}", v_impair);
    println!("Les 5 premiers nombres impairs sont : {:?}", sub_v)
}

fn main() {
    let mut rng = rand::rng();
    let taille:usize = saisie_taille();

    let vecteur:Vec<i32> = (0..=taille).map(|_| rng.random_range(0..=100)).collect();
    let vecteur_impair:Vec<i32> = vecteur.iter().cloned().filter(|&x| x % 2 != 0).collect();
    let sub_vecteur: Vec<i32> = vecteur_impair.iter().take(5).cloned().collect();

    affichage(vecteur, vecteur_impair, sub_vecteur);
}