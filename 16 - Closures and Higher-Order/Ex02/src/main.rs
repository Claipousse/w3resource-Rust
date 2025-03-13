use std::io;

fn input_number(message:&str) -> i32 {
    loop {
        let mut n = String::new();
        println!("{}", message);
        io::stdin().read_line(&mut n).expect("Failed to read line");
        match n.trim().parse::<i32>() {
            Ok(n) => return n,
            Err(_) => println!("Error: Invalid input. Please try again.")
        }
    }
}

fn apply_closure<F:FnOnce(i32, i32) -> i32>(a:i32, b:i32, closure:F) -> i32 {
    closure(a, b)
}

fn main() {
    let a:i32 = input_number("Input number A");
    let b:i32 = input_number("Input number B");
    let sum_closure = |a:i32, b:i32| a + b;
    let sum = apply_closure(a, b , sum_closure);
    println!("{a} + {b} = {sum}");
}