use rand::Rng;

fn affichage(tableau:[i32; 9], tableau_impair:Vec<i32>) {
    println!("Tableau : {:?}", tableau);
    println!("Tableau sans nombres pairs : {:?}",tableau_impair)
}

fn main() {
    let mut rng = rand::rng();
    let tableau:[i32; 9] = [(); 9].map(|_| rng.random_range(0..=100));
    let tableau_impair:Vec<i32> = tableau.iter().cloned().filter(|&x| x % 2 != 0).collect();
    affichage(tableau,tableau_impair)
}