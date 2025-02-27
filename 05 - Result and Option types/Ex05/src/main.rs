use std::io;

fn saisie_date() -> i32 {
    loop {
        let mut date = String::new();
        println!("Saisissez une date (Format JJ-MM-AAAA) :");
        io::stdin().read_line(&mut date).expect("Erreur de lecture...");
        match date.trim().parse::<i32>() {
            Ok(n)
        }
    }
}

fn main() {
    println!("Hello, world!");
}
