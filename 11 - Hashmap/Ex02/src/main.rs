use::std::collections::HashMap;

fn main() {
    let mut jeux:HashMap<String, i32> = HashMap::new();
    jeux.insert(String::from("OneShot"), 2014);
    jeux.insert(String::from("Doom"), 1993);
    jeux.insert(String::from("Half-Life"), 1998);

    for (nom, date_sortie) in &jeux {
        println!("Le jeu {} est sorti en {}",nom, date_sortie)
    }
}
