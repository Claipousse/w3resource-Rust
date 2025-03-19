use std::io;
use rand::Rng;

fn input_size() -> usize {
    loop {
        let mut input = String::new();
        println!("How many integers you want in the vector ? (Max 10) :");
        io::stdin().read_line(&mut input).expect("Failed to read line");
        match input.trim().parse::<usize>() {
            Ok(n) if n <= 10 => return n,
            Ok(n) if n > 10 => println!("Error : The maximum is 10. Please try again."),
            _ => println!("Error : Invalid input. Please try again.")
        }
    }
}

fn main() {
    let mut rng = rand::rng();
    let size = input_size();
    let vec:Vec<i32> = (0..size).map(|_| rng.random_range(0..100)).collect();

    for integer in vec {
        println!("{}", integer)
    }
}
