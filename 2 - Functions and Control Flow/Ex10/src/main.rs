use std::io;

fn saisie_string() -> String {
    let mut str = String::new();
    println!("Saisissez une chaine de caractère :");
    io::stdin().read_line(&mut str).expect("Erreur de lecture...");
    str.trim().to_string() //Supprime espaces & retours à la ligne
}

fn inversement_str(str: &str) -> String {
    str.chars().rev().collect()
    /* chars() : transforme string en suite de char
     * rev() : inverse l'ordre de la suite de caractères
     * collect() : réassemble les char en string
     */
}

fn affichage_palindrome(str:&str, str_inversee:&str) {
    if str == str_inversee  {
        println!("✔️ '{}' est un palindrome",str)
    }
    else {
        println!("❌ '{}' n'est pas un palindrome, en effet : '{}' est différent de '{}'", str, str_inversee, str)
    }
}

fn main() {
    let str = saisie_string();
    let str_inversee = inversement_str(&str);
    affichage_palindrome(&str, &str_inversee);
}