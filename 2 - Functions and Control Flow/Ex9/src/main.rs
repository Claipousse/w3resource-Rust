use std::io;

fn saisie_taille() -> usize {
    let mut input = String::new();
    println!("Combien d'éléments voulez-vous dans le tableau?");
    io::stdin().read_line(&mut input).expect("Erreur de lecture...");
    let taille_tableau: usize = input.trim().parse().expect("Veuillez entrer un nombre valide !");
    taille_tableau
}

fn saisie_tableau(taille_tableau:usize) -> Vec<f32> {
    let mut tableau = Vec::new();
    for i in 0..taille_tableau {
        loop {
            println!("Saisissez l'élément {} du tableau :", i + 1);
            let mut n_str = String::new();
            io::stdin().read_line(&mut n_str).expect("Erreur de lecture...");
            match n_str.trim().parse::<f32>() {
                Ok(n) => {
                    tableau.push(n);
                    break
                }
                Err(_) => {
                    println!("Veuillez saisir un nombre valide !");
                    continue
                }
            }
        }
    }
    tableau
}

fn somme_tableau(tableau: &Vec<f32>) -> f32 {
    tableau.iter().sum()
}

fn affichage(tableau: &Vec<f32>, somme:f32) {
    println!();
    println!("Tableau saisi : {}", tableau.iter().map(|x| x.to_string()).collect::<Vec<_>>().join(", "));
    /* Pour avoir un affichage sans les crochets :
     * .iter() : itère sur le vecteur
     * .map(|x| x.to_string()) : Convertit chaque f32 -> String
     * .collect::<Vec<_>>() : Transforme en Vec<String>.
     * .join(", ") : Fusionne les éléments avec comme séparateur ','
     */
    println!();
    println!("Somme des éléments : {}", somme)
}

fn main() {
    let taille:usize = saisie_taille();
    let tableau = saisie_tableau(taille);
    let somme: f32 = somme_tableau(&tableau);
    affichage(&tableau, somme)
}