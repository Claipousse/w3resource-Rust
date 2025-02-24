use std::collections::HashMap;

fn main() {
    let vecteur_tuples = vec![("Pomme", 3), ("Banane", 5), ("Orange", 2)];
    let mut hashmap_from_tuples: HashMap<&str, i32> = HashMap::new();

    for (key, value) in vecteur_tuples {
        hashmap_from_tuples.insert(key, value);
    }

    println!("HashMap from tuples: {:?}", hashmap_from_tuples);
}