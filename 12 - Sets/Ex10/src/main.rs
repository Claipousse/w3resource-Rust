use std::collections::HashSet;

fn perform_set_operation(set1:&HashSet<i32>, set2:&HashSet<i32>, operation:char) -> HashSet<i32> {
    match operation {
        'u' => set1.union(set2).cloned().collect(),
        'i' => set1.intersection(set2).cloned().collect(),
        'd' => set1.difference(set2).cloned().collect(),
        's' => set1.symmetric_difference(set2).cloned().collect(),
        _ => HashSet::new(),
    }
}

fn main() {
    let set1: HashSet<i32> = vec![1, 2, 3].into_iter().collect();
    let set2: HashSet<i32> = vec![3, 4, 5].into_iter().collect();

    // Perform set operations and print the results
    println!("Union : {:?}", perform_set_operation(&set1, &set2, 'u'));
    println!("Intersection : {:?}", perform_set_operation(&set1, &set2, 'i'));
    println!("Différence : {:?}", perform_set_operation(&set1, &set2, 'd'));
    println!("Différence Symétrique : {:?}", perform_set_operation(&set1, &set2, 's'));
}