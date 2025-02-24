fn main() {
    let vecteur:Vec<u8> = (1..=10).collect();
    if let Some(valeur) = vecteur.get(3) {
        println!("Le 4ème élément du vecteur est : {}", valeur)
    }
    else {
        println!("L'élément n'existe pas")
    }
}
