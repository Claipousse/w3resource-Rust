use std::{io, thread};
use std::sync::{Arc, Mutex};

fn nombre() -> u32 {
    loop {
        let mut n = String::new();
        println!("Saisissez un entier positif :");
        io::stdin().read_line(&mut n).expect("Erreur de lecture...");
        match n.trim().parse::<u32>() {
            Ok(n) => return n,
            Err(_) => println!("Erreur : La siasie est incorrecte")
        }
    }
}

fn main() {
    let n:u32 = nombre();
    let factorielle = Arc::new(Mutex::new(1));
    let mut handles = vec![];

    for i in 1..=n {
        let factorielle = Arc::clone(&factorielle);
        let handle = thread::spawn(move || {
            let mut factorielle = factorielle.lock().unwrap();
            *factorielle *= i;
        });
        handles.push(handle)
    }

    for handle in handles {
        handle.join().unwrap();
    }

    let factorielle = factorielle.lock().unwrap();
    println!("La factorielle de {} est : {}", n, factorielle)
}