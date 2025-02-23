use rand::{rng, Rng};

fn somme(tableau:[i32; 7]) -> i32 {
    let somme:i32 = tableau.iter().sum();
    somme
}

fn affichage(tableau:[i32; 7], somme:i32) {
    println!("Tableau : {:?}", tableau);
    println!("Somme : {}", somme)
}

fn main() {
    let mut rng = rng();
    let tableau:[i32; 7] =[(); 7].map(|_| rng.random_range(1..=100));
    let somme:i32 = somme(tableau);
    affichage(tableau,somme)
}