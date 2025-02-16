use std::io;

fn saisie_coordonnees() -> (f64, f64) {
    loop {
        let mut coordonnees = String::new();
        println!("Saisissez des coordonnées (latitude/longitude) :");
        io::stdin().read_line(&mut coordonnees).expect("Erreur de lecture...");

        let coordonnees:Vec<&str> = coordonnees.trim().split('/').collect();
        if coordonnees.len() != 2 {
            println!("Format incorrect. Veuillez entrer exactement une latitude et une longitude séparées par un seul '/' ");
            continue;
        }
        let latitude: Result<f64, _> = coordonnees[0].trim().parse();
        let longitude: Result<f64, _> = coordonnees[1].trim().parse();

        match (latitude, longitude) {
            (Ok(lat), Ok(lon)) => return (lat, lon), // Sortir de la boucle si la saisie est valide
            _ => println!("Entrée invalide. Veuillez réessayer."),
        }
    }
}

fn affichage(coordonnees:(f64,f64)) {
    println!("Coordonnées saisies : {},{}", coordonnees.0, coordonnees.1);
}

fn main() {
    let coordonnees = saisie_coordonnees();
    affichage(coordonnees)
}