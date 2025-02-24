use rand::Rng;

fn carre(tableau:[i32; 5]) -> Vec<i32> {
    let tableau_carre:Vec<i32> = tableau.iter().cloned().map(|x:i32| x * x).collect();
    tableau_carre
}
fn affichage(tableau:[i32; 5], tableau_carre:Vec<i32>) {
    println!("Tableau : {:?}", tableau);
    println!("Tableau avec carré : {:?}", tableau_carre)
}

fn main() {
    let mut rng = rand::rng();
    let tableau:[i32; 5] = [(); 5].map(|_| rng.random_range(0..=10));
    let tableau_carre = carre(tableau);
    affichage(tableau,tableau_carre)
}