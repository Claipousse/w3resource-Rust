use std::io;
use std::collections::HashMap;

fn saisie_jeu() -> String {
    loop{
        let mut jeu = String::new();
        println!("Saisissez le nom du jeu à ajouter au HashMap :");
        io::stdin().read_line(&mut jeu).expect("Erreur de lecture");
        let jeu = jeu.trim().to_string();
        if jeu.is_empty() {
            println!("Le nom saisit est vide. Veuillez Rééssayer");
            continue
        }
        return jeu;
    }
}

fn saisie_annee() -> i32 {
    loop {
        let mut annee = String::new();
        println!("Saisissez l'annéee de sortie du jeu :");
        io::stdin().read_line(&mut annee).expect("Erreur de lecture");
        match annee.trim().parse::<i32>() {
            Ok(n) if n >= 1972 => return n, //1972 = Date sortie de Pong
            _ =>  println!("Veuillez saisir une année correcte !")
        }
    }
}

fn main() {
    let mut jeux = HashMap::new();
    jeux.insert(String::from("OneShot"), 2014);
    jeux.insert(String::from("Doom"), 1993);
    jeux.insert(String::from("Quake"), 1996);
    jeux.insert(String::from("Omori"), 2020);

    let jeu_ajoute:String = saisie_jeu();
    let annee:i32 = saisie_annee();
    jeux.insert(jeu_ajoute, annee);

    for (jeu, annee) in &jeux {
        println!("{} ({})", jeu, annee)
    }
}