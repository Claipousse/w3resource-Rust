use std::io;
use rand::Rng;

fn input_size() -> usize {
    loop {
        let mut size = String::new();
        println!("How many tuple you want ?");
        io::stdin().read_line(&mut size).expect("Failed to read line");
        match size.trim().parse::<usize>() {
            Ok(n) => return n,
            Err(_) => println!("Error : Invalid input. Please try again.")
        }
    }
}

fn apply_closure<F, T, U>(tuples:Vec<(T, U)>, closure: F) -> Vec<(T, U)>
where
    F:Fn(T, U) -> (T, U)
{
    tuples.into_iter().map(|(t, u)| closure(t, u)).collect()
}

fn main() {
    let mut rng = rand::rng();
    let size = input_size();

    let tuples:Vec<(i32, char)> = (0..size).map(|_| {
        let x: i32 = rng.random_range(0..100);
        let y: char = (rng.random_range(97..=122) as u8) as char;
        (x,y)
    }).collect();

    let modified_tuples = apply_closure(tuples.clone(), |x:i32, y:char| (x * 2, y.to_ascii_uppercase()));
    println!("{:?}", tuples);
    println!("{:?}", modified_tuples);
}