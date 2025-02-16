use std::io;

fn saisie_prenom() -> String {
    let mut str = String::new();
    println!("Saisissez votre prénom :");
    io::stdin().read_line(&mut str).expect("Erreur de lecture...");
    str.trim().to_string() //Supprime espaces & retours à la ligne
}

fn affichage(prenom:String) {
    println!("Hello, {} !", prenom)
}

fn main() {
    let prenom:String = saisie_prenom();
    affichage(prenom)
}
