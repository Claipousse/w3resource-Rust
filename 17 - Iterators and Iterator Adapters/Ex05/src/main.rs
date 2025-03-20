use rand::Rng;
use std::io;

fn input_size() -> usize {
    loop {
        let mut input  = String::new();
        println!("How many booleans do you want ?");
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
    let booleans:Vec<bool> = (0..size).map(|_| rng.random()).collect();

    let (mut true_count, mut false_count) = (0, 0);
    for bool in booleans {
        if bool == true {
            true_count += 1;
        } else {
            false_count += 1;
        }
    }
    println!("There is {} true and {} false.", true_count, false_count);
}
