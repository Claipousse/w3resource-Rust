use std::collections::HashSet;

fn taille_set(set:HashSet<i32>) -> usize {
    set.len()
}

fn main() {
    let set:HashSet<i32> = vec![1, 2, 3, 4, 5].into_iter().collect();
    let taille = taille_set(set);
    println!("Taille du set : {}", taille)
}