use std::io;

fn input_i() -> usize {
    loop {
        let mut i = String::new();
        println!("Which power of 2 do you want?");
        io::stdin().read_line(&mut i).expect("Error when reading the line");
        match i.trim().parse::<usize>() {
            Ok(n) => return n,
            Err(_) => println!("Error : Invalid Input. Please try again.")
        }
    }
}

fn to_superscript(i:usize) -> String {
    let superscripts = ["⁰", "¹", "²", "³", "⁴", "⁵", "⁶", "⁷", "⁸", "⁹"];
    i.to_string().chars().map(|c| superscripts[c.to_digit(10).unwrap() as usize]).collect()
}


fn apply_closure<F, T>(mut closure: F, mut value: T, i:usize) -> T
where
    F: FnMut(T) -> T,
    T: Copy,
{
    for _ in 0..i {
        value = closure(value)
    }
    value
}

fn main() {
    let i:usize = input_i();
    let result = apply_closure(|x:i32| x * 2, 2, i - 1);
    println!("2{} = {}", to_superscript(i), result)
}
