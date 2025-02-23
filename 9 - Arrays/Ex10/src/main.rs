use rand::Rng;

fn tri_croissant(tab:[i32; 10]) -> [i32; 10] {
    let mut tab_croissant:[i32; 10] = tab; //On fait une copie du tableau, on va filtrer cette copie, pour pouvoir afficher les deux tableaux
    tab_croissant.sort();
    tab_croissant
}

fn affichage(tab:[i32; 10], tab_croissant:[i32; 10]) {
    println!("Tableau : {:?}", tab);
    println!("Tableau croissant : {:?}", tab_croissant)
}

fn main() {
    let mut rng = rand::rng();
    let tab:[i32; 10] = [(); 10].map(|_| rng.random_range(0..=200));
    let tab_croissant = tri_croissant(tab);
    affichage(tab, tab_croissant);
}