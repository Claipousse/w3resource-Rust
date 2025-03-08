use rand::Rng;
use std::io;

fn main() {
    let mut rng = rand::rng();
    let tableau:[i32; 100] =  std::array::from_fn(|_| rng.random_range(1..=200));

    println!("{:?}", tableau)
}
