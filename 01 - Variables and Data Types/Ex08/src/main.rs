use std::io;

fn saisie_tableau() -> [i32; 5]{
    let mut tableau:[i32; 5] = [0; 5];
    for i in 0..5 {
        loop {
            let mut saisie = String::new();
            println!("Saisissez la valeur numéro {} :", i+1);
            io::stdin().read_line(&mut saisie).expect("Erreur de lecture...");
            match saisie.trim().parse::<i32>() {
                Ok(n) => {
                    tableau[i] = n;
                    break;
                }
                Err(_) => println!("Veuillez saisir un entier !")
            }
        }
    }
    tableau
}

fn affichage(tab:[i32; 5]) {
    println!(); //Saut à la ligne
    for i in 0..5 {
        println!("{}", tab[i])
    }
}

fn main() {
    let tableau:[i32; 5] = saisie_tableau();
    affichage(tableau)
}