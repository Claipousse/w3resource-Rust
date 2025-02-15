use std::io;

fn metre_ou_centimetre() -> bool {
    loop {
        let mut choix = String::new();
        println!("Voulez-vous une circonférence en mètre ou en centimètre (m/cm) ?");
        io::stdin().read_line(&mut choix).expect("Erreur de lecture...");
        let choix = choix.trim().to_lowercase(); // Suppression des espaces et conversion en minuscule

        match choix.as_str() {
            "m" => return true,  // Retourne `true` pour les mètres
            "cm" => return false, // Retourne `false` pour les centimètres
            _ => println!("Veuillez entrer 'm' pour mètres ou 'cm' pour centimètres."),
        }
    }
}
fn saisie_diametre(unite_mesure:bool) -> f64 {
    loop {
        let mut diametre = String::new();
        if unite_mesure {
            println!("Saisissez le diamètre du cercle (en mètres) :");
        } else {
            println!("Saisissez le diamètre du cercle (en centimètres) :");
        }
        io::stdin().read_line(&mut diametre).expect("Erreur de lecture...");
        match diametre.trim().parse::<f64>() {
            Ok(d) if d > 0.0 => return d,
            _ => println!("Saisissez un diamètre positif !")
        };
    }
}

fn calcul_circonference(diametre:f64, pi:f64, unite_mesure:bool) {
    if unite_mesure {
        println!("La circonférence du cercle est de : {} mètres", diametre * pi)
    }
    else {
        println!("La circonférence du cercle est de : {} centimètres", diametre * pi)
    }
}

fn main() {
    let unite_mesure = metre_ou_centimetre();
    let pi = std::f64::consts::PI;
    let diametre:f64 = saisie_diametre(unite_mesure);
    calcul_circonference(diametre, pi, unite_mesure);
}
