use std::io;

fn saisie_nombre() -> u64 {
    loop {
        let mut n = String::new();
        println!("Saisissez un entier positif :");
        io::stdin().read_line(&mut n).expect("Erreur de lecture...");
        match n.trim().parse::<u64>() {
            Ok(n) if n == 0 => println!("Veuillez saisir un nombre positif !"),
            Ok(n) if n > 93 => println!("La suite de Fibonacci ne peut pas être réalisée pour un nombre supérieur à 93, veuillez réessayer."), //Car sinon erreur de panique de Rust
            Ok(n) => return n,
            Err(_) => println!("Entrée invalide. Veuillez saisir un nombre entier valide !"),
        }
    }
}

fn fibonacci(n: u64) -> u64 {
    //Cas particulier
    if n == 1 {
        return 1;
    }
    //On fait un tuple de a et b pour sauvegarder les valeurs de la suite
    let mut fib = (0, 1);
    //Equivalent de b = a + b, qu'on répète n fois
    for _ in 2..=n {
        fib = (fib.1, fib.0 + fib.1);
    }
    //On retourne la valeur de b
    fib.1
}

fn affichage(n:u64, resultat:u64) {
    if n == 1 {
        println!("Le 1er nombre de la suite de Fibonacci est 1"); //'er' != 'ème'
    }
    else {
        println!("Le {}ème nombre de la suite de Fibonacci est {}", n, resultat)
    }
}

fn main() {
    let n = saisie_nombre();
    let resultat = fibonacci(n);
    affichage(n, resultat)
}