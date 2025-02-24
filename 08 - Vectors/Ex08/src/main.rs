fn main() {
    let vecteur1:Vec<u8> = (1..=5).collect();
    let vecteur2:Vec<u8> = (6..=10).collect();
    //On peut aussi utiliser : vecteur1.extend(vecteur2) à condition de rendre vecteur1 mutable
    let vecteur_concat = [vecteur1, vecteur2].concat();
    println!("{:?}", vecteur_concat)
}
