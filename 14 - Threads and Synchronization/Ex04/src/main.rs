use std::thread;

fn main() {
    let matrice_a = vec![
        vec![1, 2, 3],
        vec![4, 5, 6],
    ];
    let matrice_b = vec![
        vec![7, 8],
        vec![9, 10],
        vec![11, 12],
    ];

    let lignes_a =matrice_a.len();
    let colonnes_a = matrice_a[0].len();
    let colonnes_b = matrice_b[0].len();

    //Initialise le vecteur avec des 0
    let _resultat = vec![vec![0; colonnes_b]; lignes_a];
    //Crée handles pour stocker les threads
    let mut handles = vec![];

    for i in 0..lignes_a {
        let matrice_a_clone = matrice_a.clone();
        let matrice_b_clone = matrice_b.clone();
        let handle = thread::spawn(move || {
            let mut ligne_resultat = vec![0; colonnes_b];
            for j in 0..colonnes_b {
                let mut temp = 0;
                for k in 0..colonnes_a {
                    temp += matrice_a_clone[i][k] * matrice_b_clone[k][j];
                }
                ligne_resultat[j] = temp;
            }
            ligne_resultat
        });
        handles.push(handle);
    }
    let mut resultat_ligne = vec![];
    for handle in handles {
        resultat_ligne.push(handle.join().unwrap());
        println!("Résultat de la matrice de multiplication : {:?}", resultat_ligne)
    }
}