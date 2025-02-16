use std::io;

fn saisie_string() -> String {
    let mut input = String::new();
    println!("Saisissez du texte :");
    io::stdin().read_line(&mut input).expect("Erreur de lecture...");
    input.trim_end().to_string()
}

fn premier_char(texte:&String) -> Option<char> {
    texte.chars().next()
}

fn affichage(texte:&String) {
    match premier_char(&texte) {
        Some(premier_carac) => println!("Le premier caractère est : {}", premier_carac),
        None => println!("La chaine est vide!")
    }
}

fn main() {
    let texte:String = saisie_string();
    affichage(&texte)
}