fn saisie_vecteur() -> Vec<u8>{
    let mut vecteur:Vec<u8> = (1..=5).collect();
    for i in 6..=10 {
        vecteur.push(i)
    }
    vecteur.pop();
    vecteur
}

fn main() {
    let vecteur = saisie_vecteur();
    println!("Vecteur : {:?}", vecteur)
}