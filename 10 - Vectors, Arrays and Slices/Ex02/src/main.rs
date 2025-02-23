fn main() {
    let mut vecteur:Vec<char> = ('A'..='Z').collect();
    vecteur.drain(0..=7); //Supprime les 7 premiers éléments
    let vecteur_slice = &vecteur[vecteur.len() - 10..]; //Pour obtenir les 10 derniers éléments
    println!("{:?}", vecteur_slice)
}