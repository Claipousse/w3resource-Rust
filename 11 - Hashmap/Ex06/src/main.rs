use std::io;
use std::collections::HashMap;

fn saisie_jeu() -> String {
    loop{
        let mut jeu = String::new();
        println!("Saisissez le nom du jeu à supprimer :");
        io::stdin().read_line(&mut jeu).expect("Erreur de lecture");
        let jeu = jeu.trim().to_string();
        if jeu.is_empty() {
            println!("Le nom saisit est vide. Veuillez Rééssayer");
            continue
        }
        return jeu;
    }
}

fn main() {
    let mut jeux = HashMap::new();
    jeux.insert(String::from("OneShot"), 2014);
    jeux.insert(String::from("Doom"), 1993);
    jeux.insert(String::from("Quake"), 1996);
    jeux.insert(String::from("Omori"), 2020);

    let jeu_supprimer:String = saisie_jeu();

    match jeux.remove(&jeu_supprimer) {
        Some(_) => {
            println!("\nListe des jeux :");
            for (nom,annee) in &jeux {
                println!("{} ({})",nom, annee)
            }
        }
        None => println!("Erreur : '{}' n'est pas dans la liste", jeu_supprimer)
    }
}