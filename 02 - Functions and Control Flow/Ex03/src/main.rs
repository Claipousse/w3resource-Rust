use std::io;

fn saisie_taille() -> usize {
    let mut input = String::new();
    println!("Combien d'éléments voulez-vous dans le tableau?");
    io::stdin().read_line(&mut input).expect("Erreur de lecture...");
    let taille_tableau: usize = input.trim().parse().expect("Veuillez entrer un nombre valide !");
    taille_tableau
}

fn saisie_tableau(taille_tableau:usize) -> Vec<i32> {
    let mut tableau = Vec::new();
    for i in 0..taille_tableau {
        loop {
            println!("Saisissez l'élément {} du tableau :", i + 1);
            let mut n_str = String::new();
            io::stdin().read_line(&mut n_str).expect("Erreur de lecture...");
            match n_str.trim().parse::<i32>() {
                Ok(n) => {
                    tableau.push(n);
                    break
                }
                Err(_) => {
                    println!("Veuillez saisir un nombre valide !");
                    continue
                }
            }
        }
    }
    tableau
}

fn trouver_max(tableau: &[i32]) -> Option<i32> {
    tableau.iter().max().copied()
}

fn trouver_min(tableau: &[i32]) -> Option<i32> {
    tableau.iter().min().copied()
}

fn affichage(tableau: &Vec<i32>) {
    println!(); //Saut à la ligne
    match trouver_max(&tableau) {
        Some(max) => println!("Le maximum est : {}", max),
        None => println!("Le vecteur est vide !"),
    }
    println!(); //Saut à la ligne
    match trouver_min(&tableau) {
        Some(min) => println!("Le minimum est : {}", min),
        None => println!("Le vecteur est vide !")
    }
}

fn main() {
    let taille_tableau:usize = saisie_taille();
    let tableau = saisie_tableau(taille_tableau);
    affichage(&tableau);
}