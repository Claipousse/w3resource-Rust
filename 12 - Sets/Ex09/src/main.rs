use std::collections::HashSet;

fn est_identique(set1:HashSet<i32>, set2:HashSet<i32>) -> bool {
    if set1.len() != set2.len() {
        return false;
    }
    for &element in &set1 { //Pour les éléments du set 1
        if !set2.contains(&element) { //Si le set 2 contient ne contient pas l'élément i du set 1
            return false //Alors ils ne sont pas égaux
        }
    }
    true
}

fn affichage(identique:bool) {
    if identique == true {
        println!("Les deux vecteurs sont identiques !");
    } else {
        println!("Les deux vecteurs ne sont pas identiques...")
    }
}

fn main() {
    let set1 = vec![1, 2, 3].into_iter().collect();
    let set2 = vec![1, 2, 3].into_iter().collect();
    let identique:bool = est_identique(set1, set2);
    affichage(identique)
}