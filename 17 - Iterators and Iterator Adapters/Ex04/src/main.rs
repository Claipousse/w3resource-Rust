use std::io;
use std::time::Duration;
use rand::Rng;

fn input_size() -> usize {
    loop {
        let mut input = String::new();
        println!("How many tuple you want ?");
        io::stdin().read_line(&mut input).expect("Failed to read input");
        match input.trim().parse::<usize>() {
            Ok(n) => break n,
            Err(_) => println!("Error : Invalid Input. Please try again.")
        }
    }
}

fn main() {
    let mut rng = rand::rng();
    let size = input_size();

    let tuples: Vec<(i32, i32)> = (0..size).map(|_| {
        let a:i32 = rng.random_range(0..=100);
        let b:i32 = rng.random_range(0..=100);
        (a, b)
    }).collect();

    println!("Product of the first & second element of each tuples :");
    for tuple in tuples {
        std::thread::sleep(Duration::from_secs(1));
        println!("{} x {} = {}", tuple.0, tuple.1, tuple.0 * tuple.1);
    }
}
