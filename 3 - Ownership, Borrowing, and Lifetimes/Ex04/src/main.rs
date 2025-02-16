fn premier_element(vec:&[i32]) -> Option<&i32> {
    vec.first()
}

fn main() {
    let vecteur:Vec<i32> = vec![9, 6, 3, 0, 5];
    match premier_element(&vecteur) {
        Some(premier_element) => println!("Le 1er élément du vecteur est : {}.", premier_element),
        None => println!("Erreur : Le vecteur est vide...")
    }
println!("Le vecteur après l'avoir emprunté : {:?}", vecteur);
}