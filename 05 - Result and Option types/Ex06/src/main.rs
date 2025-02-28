use std::io;

fn saisie_string() -> String {
    let mut input = String::new();
    println!("Saisissez du texte :");
    io::stdin().read_line(&mut input).expect("Erreur de lecture");
    input.trim().to_string()
}

fn taille_string(string:&String) -> Option<usize> {
    if string.len() > 0 {
        Some(string.len())
    } else {
        None
    }
}

fn main() {
    let string = saisie_string();
    match taille_string(&string) {
        Some(str) => println!("Il y a {} caractères dans {}", str, string),
        None => println!("Erreur : La chaîne de caractère est vide")
    }
}