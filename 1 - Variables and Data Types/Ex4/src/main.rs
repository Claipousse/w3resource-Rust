use std::io;

fn beau_temps() -> bool {
    loop {
        let mut soleil = String::new();
        println!("Fait-il beau temps aujourd'hui (y/n) ?");
        io::stdin().read_line(&mut soleil).expect("Erreur de lecture...");
        let soleil = soleil.trim().to_lowercase();
        match soleil.as_str() {
            "y" => return true,
            "n" => return false,
            _ => println!("Veuillez saisir 'y' pour oui ou 'n' pour non !"),
        }
    }
}

fn affichage(soleil:bool) {
    println!();
    if soleil {
        println!("Il fait beau aujourd'hui 😊")
    }
    else {
        println!("Il ne fait pas beau aujourd'hui 😢")
    }
}

fn main() {
    let soleil:bool = beau_temps();
    affichage(soleil)
}