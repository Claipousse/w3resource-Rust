use std::io;
use regex::Regex;

struct Etudiant {
    nom: String,
    age: u8,
    email: String,
}

fn saisie_infos() -> Etudiant {
    let mut nom = String::new();
    let mut email = String::new();
    let nom_regex = Regex::new(r"^[a-zA-Z]+$").expect("Regex invalide");

    loop {
        nom.clear(); // Evite blocage si erreur lors d'une itération
        println!("Saisissez le prénom de l'étudiant :");
        io::stdin().read_line(&mut nom).expect("Erreur de lecture");
        nom = nom.trim().to_string();
        if nom_regex.is_match(&nom) { //Le nom doit être composé uniquement de lettres
            break;
        } else {
            println!("Erreur : Le prénom ne doit contenir que des lettres")
        }
    }

    let age: u8;
    loop {
        let mut age_input = String::new();
        println!("Saisissez l'âge de l'étudiant :");
        io::stdin().read_line(&mut age_input).expect("Erreur de lecture");
        match age_input.trim().parse::<u8>() {
            Ok(n) if n < 30 => {
                age = n;
                break;
            }
            _ => println!("Erreur : Veuillez saisir un âge correct...")
        }
    }

    loop {
        email.clear();
        println!("Saisissez l'email de l'étudiant :");
        io::stdin().read_line(&mut email).expect("Erreur de lecture");
        email = email.trim().to_string();
        if email.contains("@") { //Le mail doit contenir un @
            break;
        } else {
            println!("Erreur : Veuillez saisir un email correct...")
        }
    }
    Etudiant { nom, age, email }
}

fn affichage(etudiant: Etudiant) {
    println!("\nInformations de l'étudiant :\n");
    println!("Nom : {}", etudiant.nom);
    println!("Âge : {}", etudiant.age);
    println!("Email : {}", etudiant.email);
}

fn main() {
    let etudiant: Etudiant = saisie_infos();
    affichage(etudiant)
}