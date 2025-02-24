use::std::collections::HashMap;

fn main() {
    let mut personnes:HashMap<String, i32> = HashMap::new();
    personnes.insert(String::from("Clément"), 18);
    personnes.insert(String::from("Alice"), 18);
    personnes.insert(String::from("Ammar"), 26);

    println!("{:?}", personnes)
}