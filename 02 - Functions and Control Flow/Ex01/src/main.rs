use std::io;

fn saisie_longueur() -> f32 {
    loop {
        let mut longueur = String::new();
        println!("Saisissez la longueur du rectangle :");
        io::stdin().read_line(&mut longueur).expect("Erreur lors de la lecture...");
        match longueur.trim().parse::<f32>() {
            Ok(n) => return n,
            Err(_) => {
                println!("Saisissez un nombre !");
                continue;
            }
        };
    }
}

fn saisie_largeur() -> f32 {
    loop {
        let mut largeur = String::new();
        println!("Saisissez la largeur du rectangle :");
        io::stdin().read_line(&mut largeur).expect("Erreur lors de la lecture");
        match largeur.trim().parse::<f32>() {
            Ok(n) => return n,
            Err(_) => {
                println!("Saisissez un nombre");
                continue;
            }
        }
    }
}

fn calcul_aire(longueur:f32, largeur:f32) -> f32 {
    longueur * largeur
}

fn affichage(longueur:f32, largeur:f32, aire:f32) {
    println!("La longueur est {}, la largeur {}, et l'aire {}.", longueur, largeur, aire)
}

fn main() {
    let longueur:f32 = saisie_longueur();
    let largeur:f32 = saisie_largeur();
    let aire:f32 = calcul_aire(longueur, largeur);
    affichage(longueur, largeur, aire)
}
