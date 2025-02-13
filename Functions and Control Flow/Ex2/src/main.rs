use std::io;

fn saisie_n() -> u32 {
    let mut n = String::new();
    loop {
        println!("Saisissez un entier positif : ");
        io::stdin().read_line(&mut n).expect("Erreur de lecture...");
        match n.trim().parse::<u32>() {
            Ok(n) => return n,
            Err(_) => {
                println!("Veuillez saisir un entier positif !");
                continue;
            }
        };
    }
}

fn est_premier(n:u32) -> bool {
    if n < 2 {
        return false;
    }
    if n == 2 {
        return true;
    }
    if n % 2 == 0 {
        return false
    }
    let limite = (n as f64).sqrt() as u32; //Calcul de la racine de n
    for i in 3 ..=limite {
        if n % i == 0 {
            return false;
        }
    }
    true
}

fn affichage(n:u32, condition:bool) {
    if condition == true {
        println!("✔️ {} est un nombre premier !", n)
    }
    if condition == false {
        println!("❌ {} n'est pas un nombre premier...", n)
    }
}

fn main() {
    let n:u32 = saisie_n();
    let condition:bool = est_premier(n);
    affichage(n, condition)
}