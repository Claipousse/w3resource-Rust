use rand::Rng;

fn concatener(t1:[i32; 4], t2:[i32; 4]) -> Vec<i32> {
    [t1.as_slice(), t2.as_slice()].concat()
}

fn affichage(t1:[i32; 4], t2:[i32; 4], tableau_concat:Vec<i32>) {
    println!("Tableau 1 : {:?}",t1);
    println!("Tableau 2 : {:?}", t2);
    println!("Concaténation des tableaux : {:?}", tableau_concat)
}
fn main() {
    let mut rng = rand::rng();
    let tableau1:[i32; 4] = [(); 4].map(|_| rng.random_range(0..=100));
    let tableau2:[i32; 4] = [(); 4].map(|_| rng.random_range(0..=100));
    let tableau_concat = concatener(tableau1, tableau2);
    affichage(tableau1, tableau2, tableau_concat)
}
