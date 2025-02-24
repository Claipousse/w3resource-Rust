fn longueur_vecteur(vecteur:Vec<u32>) -> usize {
    let taille:usize = vecteur.len();
    taille
}

fn main() {
    let vecteur:Vec<u32> = vec![1, 3, 5, 7, 9];
    let taille_vecteur:usize = longueur_vecteur(vecteur);
    println!("Taille du vecteur : {} éléments.", taille_vecteur)

}