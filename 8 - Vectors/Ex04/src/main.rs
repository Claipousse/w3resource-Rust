use std::io;

fn saisie_facteur() -> i32{
    loop {
        let mut facteur = String::new();
        println!("Par combien voulez-vous multiplier le vecteur ?");
        io::stdin().read_line(&mut facteur).expect("Erreur lors de la lecture...");
        match facteur.trim().parse::<i32>() {
            Ok(n) => return n,
            Err(_) => println!("Saisissez un facteur correct !")
        }
    }
}

fn multiplier(vecteur:&mut Vec<i32>, facteur:i32) {
    for valeur in vecteur.iter_mut() {
        *valeur *= facteur;
    }
}

fn affichage(vecteur:Vec<i32>, facteur:i32) {
    println!("Vecteur après multiplication par {} : {:?}", facteur, vecteur)
}

fn main() {
    let mut vecteur:Vec<i32> = (1..=5).collect();
    println!("Vecteur : {:?}", vecteur);
    let facteur:i32 = saisie_facteur();
    multiplier(&mut vecteur, facteur);
    affichage(vecteur, facteur);
}