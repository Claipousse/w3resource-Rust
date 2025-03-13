use std::io;

fn input_vector_string() -> Vec<String> {
    let mut vec:Vec<String> = Vec::new();
    let mut input = String::new();

    println!("Input the words you want ('exit' to leave) :");
    loop {
        input.clear();
        io::stdin().read_line(&mut input).expect("Error when reading the line...");
        let input = input.trim().to_string();
        let input_clone = input.clone().to_lowercase(); //Exit,exit,ExIT,... works with this method to leave the loop
        if input_clone == "exit" {
            return vec;
        } else {
            vec.push(input)
        }
    }
}

fn caesar_code_key() -> u8 {
    loop {
        let mut key = String::new();
        println!("Enter the Cesar key you want ('info' for more informations) :");
        io::stdin().read_line(&mut key).expect("Error when reading the line...");
        let input_string = key.trim().to_string().to_lowercase();
        if input_string == "info" {
            println!("The Caesar cipher is a simple substitution cipher that shifts letters in the alphabet by a fixed number.\n\
            The key is the number of positions each letter is shifted.")
        }
        else {
            match key.trim().parse::<u8>() {
                Ok(n) if n > 0 && n <= 25 => return n,
                Ok(n) if n == 0 => println!("Error : If the key is 0, nothing change. Please try again."),
                Ok(n) if n > 25 => println!("Error : The maximum value of the Caesar key is 25 (A -> Z)"),
                _ => println!("Error : Invalid input. Please try again.")
            }
        }
    }
}

fn main() {
    let vec:Vec<String> = input_vector_string();
    let key = caesar_code_key();
    println!("{:?}", vec);
    println!("{key}")
}
