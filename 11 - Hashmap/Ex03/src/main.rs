use::std::collections::HashMap;
use std::io;

fn jeu_recherche() -> String {
    loop {
        let mut jeu = String::new();
        println!("Quel jeu recherchez-vous ?");
        io::stdin().read_line(&mut jeu).expect("Erreur de lecture...");
        let jeu = jeu.trim().to_string();
        if jeu.is_empty() {
            println!("Vous n'avez rien entré. Veuillez réesayer");
            continue
        }
        return jeu
    }
}

fn main() {
    let mut jeux: HashMap<String, i32> = HashMap::new();
    jeux.insert(String::from("OneShot"), 2014);
    jeux.insert(String::from("Doom"), 1993);
    jeux.insert(String::from("Half-Life"), 1998);

    let jeu_recherche:String = jeu_recherche();

    if jeux.contains_key(&jeu_recherche) {
        println!("{} est bien présent dans la liste des jeux", jeu_recherche)
    } else {
        println!("{} n'est pas présent dans la liste des jeux", jeu_recherche)
    }
}