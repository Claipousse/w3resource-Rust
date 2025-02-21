fn main() {
    let vecteur:Vec<u8> = (1..=5).collect();
    let vecteur_cube: Vec<u8> = vecteur.iter().map(|&x| x.pow(3)).collect();
    println!("Vecteur : {:?}", vecteur);
    println!("Vecteur au cube : {:?}", vecteur_cube)
}