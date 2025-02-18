use rand::Rng;
#[derive(Debug)]
enum Forme {
    Triangle,
    Cercle,
    Carre
}

fn choix_aleatoire() -> Forme {
    let mut rng = rand::rng();
    let index = rng.random_range(0..3);
    match index {
        0 => Forme::Triangle,
        1 => Forme::Cercle,
        2 => Forme::Carre,
        _ => unreachable!(),
    }
}

fn description(forme:&Forme) -> &'static str {
    match forme {
        Forme::Triangle => "Forme géométrique à 3 côtés",
        Forme::Cercle => "Forme géométrique ronde sans angles",
        Forme::Carre => "Forme géométrique à 4 côtés égaux"
    }
}

fn affichage(forme:Forme, description:&str) {
    println!("\nLa forme choisie est le {:?}", forme);
    println!("Description : {}", description)
}

fn main() {
    let forme:Forme = choix_aleatoire();
    let description: &str = description(&forme);
    affichage(forme, description)
}