use std::io;

fn saisie_a() -> i32 {
    loop {
        let mut a = String::new();
        println!("Saisissez un entier A :");
        io::stdin().read_line(&mut a).expect("Erreur de lecture...");
        match a.trim().parse::<i32>() {
            Ok(n) => return n,
            Err(_) => println!("Veuillez saisir un entier !")
        }
    }
}

fn saisie_b() -> i32 {
    loop {
        let mut b = String::new();
        println!("Saisissez un entier B :");
        io::stdin().read_line(&mut b).expect("Erreur de lecture...");
        match b.trim().parse::<i32>() {
            Ok(n) => return n,
            Err(_) => println!("Veuillez saisir un entier !")
        }
    }
}

fn somme(a:&i32, b:&i32) -> i32 {
    *a + *b
}

fn main() {
    let a:i32 = saisie_a();
    let b:i32 = saisie_b();
    let somme:i32 = somme(&a,&b);
    println!("{} + {} = {}", a, b, somme)
}