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

fn affichage(vecteur:&Vec<f64>, vecteur_racine:Vec<f64>, sub_vecteur:&[f64]) {
    println!("Vecteur : {:?}", vecteur);
    println!("Racine carré : {:?}",vecteur_racine);
    println!("Vecteur slicé : {:?}", sub_vecteur)
}

fn main() {
    let mut rng = rand::rng();
    let taille:usize = saisie_taille();
    let vecteur:Vec<f64> = (0..taille)
        .map(|_| {
            let n:f64 = rng.random_range(0.0..=50.0);
            (n * 100.0).round() / 100.0
        })
        .collect();

    let vecteur_racine:Vec<f64> = vecteur.iter().map(|&x| x.sqrt()).collect();
    let sub_vecteur = if taille > 6 { &vecteur[2..7] } else { &vecteur[..] };
    affichage(&vecteur, vecteur_racine, sub_vecteur);
}