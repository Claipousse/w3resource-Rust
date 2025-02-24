use rand::Rng;

fn affichage(tableau:[i32; 7], tableau_inversee:[i32; 7]) {
    println!("Tableau : {:?}", tableau);
    println!("Tableau inversé : {:?}", tableau_inversee)
}

fn main() {
    let mut rng = rand::rng();
    let tableau:[i32; 7] = [(); 7].map(|_| rng.random_range(0..=100));
    let mut tableau_inversee= tableau;
    tableau_inversee.reverse();
    affichage(tableau, tableau_inversee)
}