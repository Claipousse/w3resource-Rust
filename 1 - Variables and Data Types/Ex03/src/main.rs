use std::io;

fn saisie_incrementation() -> u32 {
    loop {
        let mut input = String::new();
        println!("Saisissez un entier positif :");
        io::stdin().read_line(&mut input).expect("Erreur de lecture...");
        match input.trim().parse::<u32>() {
            Ok(n) => return n,
            Err(_) => println!("Saisissez un entier positif !")
        }
    }
}

fn incrementation(a: &mut u32, b:u32) {
    *a += b
}

fn main() {
    let mut a:u32 = 0;
    let temp:u32 = a;
    let b:u32 = saisie_incrementation();
    incrementation(&mut a, b);
    println!("{} + {} = {}", temp, b, a)
}