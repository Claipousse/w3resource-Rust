use std::io;

fn input_size(message:&str) -> i32 {
    loop {
        let mut input = String::new();
        println!("{message}");
        io::stdin().read_line(&mut input).expect("Failed to read line");
        match input.trim().parse::<i32>() {
            Ok(n) => return n,
            Err(_) => println!("Error : Invalid input. Please try again.")
        }
    }
}

fn main() {
    let start = input_size("Where the range start ?");
    let end = input_size("Where the range end ?");
    if start > end {
        eprintln!("Error : Impossible to have a range with a greater start than end.")
    }
    else {
        let sum: i32 = (start..=end).sum();
        println!("Sum of the numbers {} to {} is : {}", start, end, sum);
    }
}