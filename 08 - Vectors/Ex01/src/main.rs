fn saisie_valeurs() -> Vec<u8> {
    let mut vecteur:Vec<u8> = Vec::new();
    for i in 0..10 {
        vecteur.push(i+1);
    }
    vecteur
}

fn main() {
    let vecteur:Vec<u8> = saisie_valeurs();
    println!("Vecteur : {:?}", vecteur)
}