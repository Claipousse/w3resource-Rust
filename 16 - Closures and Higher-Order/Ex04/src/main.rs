use std::io;
use rand::Rng;

fn input_size() -> usize {
    loop {
        let mut size = String::new();
        println!("How many integer you want in the vector ?");
        io::stdin().read_line(&mut size).expect("Failed to read line");
        match size.trim().parse::<usize>() {
            Ok(n) => return n,
            Err(_) => println!("Error : Invalide size. Please try again.")
        }
    }
}

fn apply_closure<F>(vec:Vec<i32>, predicate:F) -> Vec<i32>
where
    F: Fn(i32) -> bool,
{
    vec.into_iter().filter(|&x| predicate(x)).collect()
}

fn main() {
    let mut rng = rand::rng();
    let size = input_size();
    let vec:Vec<i32> = (0..size).map(|_| rng.random_range(0..=100)).collect();

    let even_predicate = |x:i32| x % 2 == 0;
    let even_vec = apply_closure(vec.clone(), even_predicate);
    println!("Vecteur : {:?}", vec);
    println!("Vecteur pair : {:?}", even_vec)
}
