use std::collections::HashMap;

fn main() {
    let mut hashmap:HashMap<String, Vec<i32>> = HashMap::new();
    let data = vec![
        ("pair", 2),
        ("impair", 3),
        ("pair", 4),
        ("impair", 5),
        ("pair", 6),
        ("impair", 7),
    ];
    for (cle, valeur) in data {
        if let Some(vec) = hashmap.get_mut(cle) {
            vec.push(valeur);
        } else {
            hashmap.insert(cle.to_string(), vec![valeur]);
        }
    }
    println!("HashMap: {:?}", hashmap);
}