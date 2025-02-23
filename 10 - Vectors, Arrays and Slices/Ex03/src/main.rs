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

fn affichage(v:&Vec<i32>, v3:&[i32]) {
    println!("\nVecteur (trié par ordre croissant) : {:?}", v);
    println!("Les 3 plus grandes valeurs sont donc {}, {} et {}", v3[0], v3[1], v3[2])
}

fn main() {
    let mut rng = rand::rng();
    let taille:usize = saisie_taille();
    let mut vecteur:Vec<i32> = (0..taille).map(|_| rng.random_range(0..=100)).collect();
    vecteur.sort();
    let vecteur_top3 = &vecteur[vecteur.len() - 3..];
    affichage(&vecteur, vecteur_top3)
}