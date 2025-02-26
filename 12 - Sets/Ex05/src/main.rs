use std::collections::HashSet;

fn subset(set1:&HashSet<i32>, set2:&HashSet<i32>) -> bool {
    for &element in set1 {
        if !set2.contains(&element) {
            return false
        }
    }
    true
}

fn affichage(is_subset:bool) {
    if is_subset == true {
        println!("Le 1er set est bel et bien un sub set du 2ème.");
    } else {
        println!("Le 1er set n'est pas un sub set du 2ème.")
    }
}

fn main() {
    let set1: HashSet<i32> = vec![1, 2, 3].into_iter().collect();
    let set2: HashSet<i32> = vec![1, 2, 3, 4, 5].into_iter().collect();

    let is_subset = subset(&set1, &set2);
    affichage(is_subset)
}