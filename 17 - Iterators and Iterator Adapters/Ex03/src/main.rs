use std::io;

fn input_strings() -> Vec<String> {
    let mut vec:Vec<String> = Vec::new();
    println!("Input as many strings as you want ('exit' to leave) :");
    loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read input");
        let input = input.trim().to_string();
        if input == "exit" {
            break;
        }
        else {
            vec.push(input)
        }
    }
    vec
}

fn main() {
    let strings:Vec<String> = input_strings();
    println!("{:?}", strings);

    for string in strings {
        print!("{} ", string.len())
    }
}