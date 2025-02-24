use std::collections::HashMap;
use std::io;

fn saisie_string() -> String {
    loop {
        let mut string= String::new();
        println!("Saisissez une chaîne de caractères :");
        io::stdin().read_line(&mut string).expect("Erreur de lecture");
        let string = string.trim().to_string();
        if string.is_empty() {
            println!("Erreur : La chaîne de caractères est vide");
            continue
        }
        return string
    }
}

fn main() {
    let string:String = saisie_string();
    let mut nombre_char = HashMap::new();
    for c in string.chars() {
        let i = nombre_char.entry(c).or_insert(0);
        *i += 1;
    }
    println!("{:?}", nombre_char)
}