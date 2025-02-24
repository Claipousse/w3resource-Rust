use std::io;

fn saisie_taille() -> usize {
    loop {
        let mut taille = String::new();
        println!("Combien d'entiers voulez-vous dans le vecteur ?");
        io::stdin().read_line(&mut taille).expect("Erreur de lecture...");
        match taille.trim().parse::<usize>() {
            Ok(n) => return n,
            Err(_) => println!("Veuillez saisir une taille correcte !")
        }
    }
}

fn saisie_vecteur(taille:usize) -> Vec<i32> {
    let mut vecteur = Vec::new();
    for i in 0..taille {
        loop {
            let mut input = String::new();
            println!("Saisissez l'entier numéro {} :", i+1);
            io::stdin().read_line(&mut input).expect("Erreur de lecture...");
            match input.trim().parse::<i32>() {
                Ok(n) => {
                    vecteur.push(n);
                    break;
                }
                Err(_) => {
                    println!("Veuillez saisir un entier !");
                    continue
                }
            };
        }
    }
    vecteur
}

fn redimensionner_vecteur(taille:usize, mut vecteur:Vec<i32>) -> Vec<i32> {
    vecteur.resize(taille * 2, 0);
    vecteur
}

fn affichage(vecteur:Vec<i32>) {
    println!("Longueur du vecteur doublée : {}", vecteur.len());
    println!("Vecteur : {:?}", vecteur)
}

fn main() {
    let taille:usize = saisie_taille();
    let mut vecteur:Vec<i32> = saisie_vecteur(taille);
    vecteur = redimensionner_vecteur(taille, vecteur);
    affichage(vecteur)
}