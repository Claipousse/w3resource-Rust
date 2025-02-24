enum Resultat<T, E> {
    Succès(T),
    Échec(E),
}

fn diviser(a: f64, b: f64) -> Resultat<f64, String> {
    if b == 0.0 {
        Resultat::Échec("Division par zéro".to_string())
    } else {
        Resultat::Succès(a / b)
    }
}

fn main() {
    let resultat1 = diviser(10.0, 2.0);
    let resultat2 = diviser(10.0, 0.0);

    match resultat1 {
        Resultat::Succès(valeur) => println!("Succès: {}", valeur),
        Resultat::Échec(err) => println!("Erreur: {}", err),
    }

    match resultat2 {
        Resultat::Succès(valeur) => println!("Succès: {}", valeur),
        Resultat::Échec(err) => println!("Erreur: {}", err),
    }
}