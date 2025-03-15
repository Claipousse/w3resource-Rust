use std::io;

fn input_string(message:&str) -> String {
    let mut input = String::new();
    println!("{message}");
    io::stdin().read_line(&mut input).expect("Error when reading the line.");
    input.trim().to_string()
}

fn apply_closure<F>(str1:&str, str2:&str, closure:F) -> String
where
    F:FnOnce(String) -> String
{
    let concatenated_str = format!("{}{}", str1, str2);
    closure(concatenated_str)
}

fn main() {
    let str1 = input_string("Input the first string :");
    let str2 = input_string("Input the second string :");
    let concatenate_strings_uppercase = apply_closure(&str1, &str2, |s| s.to_uppercase());
    println!("{} + {} -> {}", str1, str2, concatenate_strings_uppercase);
}
