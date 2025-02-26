use std::collections::HashSet;

fn superset(set1:&HashSet<i32>, set2:&HashSet<i32>) -> bool {
    for &element in set2 {
        if !set1.contains(&element) {
            return false
        }
    }
    true
}

fn affichage(is_subset:bool) {
    if is_subset == true {
        println!("Le 1er set est bel et bien un super du 2ème.");
    } else {
        println!("Le 1er set n'est pas un super du 2ème.")
    }
}

fn main() {
    let set1: HashSet<i32> = vec![1, 2, 3, 4, 5].into_iter().collect();
    let set2: HashSet<i32> = vec![1, 2, 3].into_iter().collect();

    let is_subset = superset(&set1, &set2);
    affichage(is_subset)
}