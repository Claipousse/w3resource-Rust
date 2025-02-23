use rand::Rng;
use std::io;

fn valeur_recherche() -> i32 {
    loop{
        let mut input = String::new();
        println!("Quelle entier recherchez-vous ?");
        io::stdin().read_line(&mut input).expect("Erreur de lecture...");
        match input.trim().parse::<i32>() {
            Ok(n) => return n,
            Err(_) => println!("Veuillez saisir un entier valide !")
        }
    }
}

fn recherche(tableau:[i32; 7], n:i32) -> bool {
    for i in 0..7 {
        if tableau[i] == n {
            return true
        }
    }
    false
}

fn affichage(presence:bool, n:i32) {
    if presence == true {
        println!("{} est présent dans le tableau !", n)
    } else {
        println!("{} n'est pas présent dans le tableau...", n)
    }
}

fn main() {
    let mut rng = rand::rng();
    let tableau:[i32; 7] = [(); 7].map(|_| rng.random_range(1..=10));
    let valeur = valeur_recherche();
    let presence:bool = recherche(tableau, valeur);
    println!("{:?}", tableau);
    affichage(presence,valeur)
}