use rand::Rng;
use std::io;

fn input_size() -> usize {
    loop {
        let mut input  = String::new();
        println!("How many floats you want ?");
        io::stdin().read_line(&mut input).expect("Failed to read line");
        match input.trim().parse::<usize>() {
            Ok(n) => return n,
            Err(_) => println!("Error : Invalid input. Please try again.")
        }
    }
}

fn main() {
    let mut rng = rand::rng();
    let size = input_size();
    let floats:Vec<f64> = (0..size).map(|_| {
        let float:f64 = rng.random_range(0.0..=1000.0);
        (float * 100.0).round() / 100.0
    }).collect();

    let mut sum:f64 = 0.0;
    let count = floats.len();

    for float in &floats {
        sum += float;
    }
    let average = sum / count as f64;

    println!("{:?}", floats);
    println!("\nAverage : {}", average);
}
