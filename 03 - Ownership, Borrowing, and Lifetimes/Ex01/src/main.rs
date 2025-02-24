use std::io;

fn saisie_string() -> String{
    let mut mon_string = String::new();
    println!("Veuillez saisir un texte :");
    io::stdin().read_line(&mut mon_string).expect("Erreur de lecture...");
    let mon_string = mon_string.trim().to_string();
    mon_string
}

fn affichage_string(s: String) {
    println!();
    println!("Texte saisi : {}", s)
}

fn main() {
    let mon_string = saisie_string();
    affichage_string(mon_string);
    //En faisant println!("{}", mon_string), Erreur car la fonction affichage_string prend l'ownership
}