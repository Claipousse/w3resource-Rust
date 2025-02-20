use std::io;
use chrono::{Datelike, Utc};

struct Voiture {
    fabricant:String,
    modele:String,
    annee:u16
}

fn saisie_infos() -> Voiture {
    let mut fabricant = String::new();
    let mut modele = String::new();
    let annee_actuelle = Utc::now().year() as u16;

    //Saisie fabricant
    fabricant.clear();
    println!("Quel est le fabricant de la voiture ?");
    io::stdin().read_line(&mut fabricant).expect("Erreur de lecture");
    fabricant = fabricant.trim().to_string();

    //Saisie modèle
    modele.clear();
    println!("Quel est le modèle de la voiture ?");
    io::stdin().read_line(&mut modele).expect("Erreur de lecture");
    modele = modele.trim().to_string();

    //Saisie année
    let annee:u16;
    loop {
        let mut annee_str = String::new();
        println!("De quelle année date la voiture ?");
        io::stdin().read_line(&mut annee_str).expect("Erreur de lecture");
        match annee_str.trim().parse::<u16>() {
            Ok(n) if n <= annee_actuelle => {
                annee = n;
                break;
            }
            _ => println!("Erreur : L'année de fabrication est invalide")
        }
    };
    Voiture{fabricant, modele, annee}
}

fn affichage(voiture:Voiture) {
    println!("\nInformations de la voiture :");
    println!("Fabricant : {}",voiture.fabricant);
    println!("Modèle : {}",voiture.modele);
    println!("Année : {}", voiture.annee)
}

fn main() {
    let voiture:Voiture = saisie_infos();
    affichage(voiture)
}