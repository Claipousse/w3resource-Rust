use std::collections::HashMap;

fn main() {
    let mut oldgen = HashMap::new();
    oldgen.insert(String::from("Doom"), 1993);
    oldgen.insert(String::from("Counter Strike Source"),2004);
    oldgen.insert(String::from("Super Mario 64"), 1996);

    let mut newgen = HashMap::new();
    newgen.insert(String::from("Cyberpunk 2077"), 2020);
    newgen.insert(String::from("Overwatch"), 2016);
    newgen.insert(String::from("Lethal Compagny"), 2023);

    let jeux:HashMap<String, i32> = oldgen.into_iter().chain(newgen.into_iter()).collect();

    for (nom, annee) in &jeux {
        println!("{} ({})",nom, annee)
    }
}