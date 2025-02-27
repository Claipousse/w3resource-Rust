use std::io;
use std::fs;

fn lire(chemin : &str) -> Result<String, io::Error> {
    fs::read_to_string(chemin)
}

fn main() {
    let chemin = "fichier.txt";
    match lire(chemin) {
        Ok(contenu) => println!("{}", contenu),
        Err(erreur) => eprintln!("Erreur de lecture : {}", erreur)
    }
}