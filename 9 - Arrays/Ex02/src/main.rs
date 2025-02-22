use rand::Rng;

fn main() {
    let mut rng = rand::rng();
    let tableau: [i32; 9] = std::array::from_fn(|_| rng.random_range(1..=100));
    println!("Tableau : {:?}", tableau);
    println!("8ème élément du tableau : {}", tableau[7])
}