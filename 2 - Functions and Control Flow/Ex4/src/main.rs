use std::io;

fn saisie_nombre() -> u32 {
    loop {
        let mut n = String::new();
        println!("Saisissez un entier positif:");
        io::stdin().read_line(&mut n).expect("Erreur de lecture...");
        match n.trim().parse() {
            Ok(n) => return n,
            Err(_) => {
                println!("Veuillez saisir un nombre entier positif !");
                continue
            }
        }
    }
}

fn calcul_factorielle(n:u32) -> u32{
    let mut factorielle:u32 = 1;
    for i in 1..=n {
        factorielle *=i;
    }
    factorielle
}

fn affichage(n:u32, factorielle:u32) {
    println!("La factorielle de {} est : {}", n, factorielle)
}

fn main() {
    let n:u32 = saisie_nombre();
    let factorielle:u32 = calcul_factorielle(n);
    affichage(n, factorielle)
}