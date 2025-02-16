use std::io;

fn saisie_string() -> String {
    let mut input = String::new();
    println!("Saisissez du texte :");
    io::stdin().read_line(&mut input).expect("Erreur de lecture...");
    input.trim_end().to_string()
}

fn longueur_string(texte:String) -> usize {
    texte.len()
}

fn main() {
    let texte:String = saisie_string();
    let taille:usize = longueur_string(texte);
    println!("Taille du texte: {} caractères.", taille)
}