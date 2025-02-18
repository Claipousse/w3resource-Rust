use std::io;
use regex::Regex;

struct Employe {
    nom:String,
    id:i32,
    metier:String,
    salaire:u32, //Mensuel
}

fn saisie_informations() -> Employe {
    let mut nom = String::new();
    let mut id = String::new();
    let mut metier = String::new();
    let mut salaire = String::new();
    let string_regex = Regex::new(r"^[\p{L}]+$").expect("Regex invalide");
    let id_regex = Regex::new(r"^\d{10}$").expect("Regex invalide");

    //Saisie nom
    loop {
        nom.clear();
        println!("Saisissez le nom de l'employé :");
        io::stdin().read_line(&mut nom).expect("Erreur de lecture...");
        nom = nom.trim().to_string();
        if string_regex.is_match(&nom) {
            break;
        } else {
            println!("Erreur : Format incorrect")
        }
    };
    //Saisie ID
    let id: i32 = loop {
        id.clear();
        println!("Saisissez l'ID de l'employé (10 chiffres) :");
        io::stdin().read_line(&mut id).expect("Erreur de lecture...");
        id = id.trim().to_string();
        if id_regex.is_match(&id) {
            match id.parse::<i32>() {
                Ok(n) => break n,
                Err(_) => println!("Erreur : L'ID doit être composé exactement de 10 chiffres"),
            }
        } else {
            println!("Erreur : L'ID doit être composé exactement de 10 chiffres")
        }
    };
    //Saisie metier
    loop {
        metier.clear();
        println!("Saisissez le métier de l'employé :");
        io::stdin().read_line(&mut metier).expect("Erreur de lecture...");
        metier = metier.trim().to_string();
        if string_regex.is_match(&metier) {
            break;
        } else {
            println!("Erreur : Format incorrect")
        }
    };
    //Saisie salaire
    let salaire:u32 = loop {
        salaire.clear();
        println!("Saisissez le salaire mensuel de l'employé (en €) :");
        io::stdin().read_line(&mut salaire).expect("Erreur de lecture...");
        match salaire.trim().parse::<u32>() {
            Ok(n) => break n,
            Err(_) => println!("Erreur : Format incorrect")
        }
    };
    Employe {nom, id, metier, salaire}
}

impl Employe {
    fn salaire_annuel(&self) -> u32 {
        self.salaire * 12
    }
}

fn affichage(employe: Employe) {
    println!("\nInformations du salarié :\n");
    println!("Nom : {}", employe.nom);
    println!("ID : {}",employe.id);
    println!("Métier : {}", employe.metier);
    println!("Salaire Mensuel : {}", employe.salaire);
    println!("Salaire Annuel : {}", employe.salaire_annuel())
}

fn main() {
    let employe:Employe = saisie_informations();
    affichage(employe)
}