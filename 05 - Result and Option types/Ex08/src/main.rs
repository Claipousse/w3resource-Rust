use std::io;

fn saisie_nombre(message:&str) -> f64 {
    loop {
        let mut n = String::new();
        println!("{}", message);
        io::stdin().read_line(&mut n).expect("Erreur de lecture...");
        match n.trim().parse::<f64>() {
            Ok(n) => return n,
            Err(_) => println!("Erreur : Veuillez saisir un entier correct")
        }
    }
}

fn division(a:f64, b:f64) -> Result<f64, &'static str>{
    if b != 0.0 {
        Ok(a / b)
    } else {
        Err("Erreur : Division par 0 impossible")
    }
}


fn main() {
    let a:f64 = saisie_nombre("Veuillez saisir le 1er entier :");
    let b:f64 = saisie_nombre("Veuillez saisir le 2ème entier :");

    match division(a, b) {
        Ok(division) => println!("{} / {} : {}", a, b, division),
        Err(erreur) => println!("{}", erreur)
    }
}
