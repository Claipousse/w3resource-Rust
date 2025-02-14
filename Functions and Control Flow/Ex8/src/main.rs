use std::io;

fn saisie_nombre() -> usize {
    loop {
        let mut taille = String::new();
        println!("Saisissez un entier positif :");
        io::stdin().read_line(&mut taille).expect("Erreur de lecture...");
        match taille.trim().parse::<usize>() {
            Ok(n) if n == 0 => println!("Veuillez saisir un nombre positif !"),
            Ok(n) if n > 93 => println!("La suite de Fibonacci ne peut pas être réalisée pour un nombre supérieur à 93, veuillez réessayer."), //Car sinon erreur de panique de Rust
            Ok(n) => return n,
            Err(_) => println!("Entrée invalide. Veuillez saisir un nombre entier valide !"),
        }
    }
}

fn fibonacci(taille: usize) -> Vec<u64> {
    let mut fib = Vec::with_capacity(taille);
    if taille > 0 {
        fib.push(0);
    }
    if taille > 1 {
        fib.push(1);
    }
    for i in 2..taille {
        let next_value = fib[i - 1] + fib[i - 2];
        fib.push(next_value);
    }
    fib
}

fn affichage(taille:usize, fib:&Vec<u64>) {
    println!("Les {} premiers nombres de Finonacci sont : {:?}",taille, &fib[..taille])
}

fn main() {
    let taille:usize = saisie_nombre();
    let fibonacci:Vec<u64> = fibonacci(taille);
    affichage(taille, &fibonacci)
}