use std::io;
use rand::rng;
use rand::seq::SliceRandom;

fn saisie_tableau() -> [i32; 5] {
    let mut tableau:[i32; 5] = [0; 5];
    for i in 0..5 {
        loop {
            let mut input = String::new();
            println!("Saisissez l'entier {} du tableau :", i + 1);
            io::stdin().read_line(&mut input).expect("Erreur de lecture...");
            match input.trim().parse::<i32>() {
                Ok(n) => {
                    tableau[i] = n;
                    break
                }
                Err(_) => {
                    println!("La valeur saisie n'est pas un entier. Veuillez réessayer.");
                    continue
                }
            };
        }
    }
    tableau
}

fn choix_aleatoire(tableau: &[i32; 5]) {
    let mut rng = rng();
    let mut indices: Vec<usize> = (0..tableau.len()).collect();
    indices.shuffle(&mut rng);

    let index1 = indices[0];
    let index2 = indices[1];
    let n1 = tableau[index1];
    let n2 = tableau[index2];

    println!("La {} ème valeur du tableau est : {}", index1 + 1, n1);
    println!("La {} ème valeur du tableau est : {}", index2 + 1, n2);
    println!("Somme de {} + {} : {}",n1, n2, n1 + n2);
}

fn main() {
    let tableau = saisie_tableau();
    choix_aleatoire(&tableau)
}