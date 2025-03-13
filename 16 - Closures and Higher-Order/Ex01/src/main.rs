use std::io;

fn input_number() -> i32 {
    loop {
        let mut n = String::new();
        println!("Input an integer :");
        io::stdin().read_line(&mut n).expect("Failed to read line");
        match n.trim().parse::<i32>() {
            Ok(n) => return n,
            Err(_) => println!("Error : Invalid number. Please try again.")
        }
    }
}

fn apply_closure<F:FnOnce(i32) -> i32>(x:i32, closure: F) -> i32 {
    closure(x)
}

fn main() {
    let number:i32 = input_number();
    let cube_closure = |x:i32| x * x * x;
    let result = apply_closure(number, cube_closure);
    println!("{number}³ = {result}")
}