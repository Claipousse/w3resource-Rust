use std::collections::HashSet;

fn difference(set1:&HashSet<i32>, set2: &HashSet<i32>) -> HashSet<i32> {
    let mut difference = HashSet::new();

    for &element in set1 {
        if !set2.contains(&element) {
            difference.insert(element);
        }
    }
    for &element in set2 {
        if !set1.contains(&element) {
            difference.insert(element);
        }
    }
    difference
}

fn main() {
    let set1: HashSet<i32> = vec![1, 2, 3].into_iter().collect();
    let set2: HashSet<i32> = vec![3, 4, 5].into_iter().collect();
    let difference = difference(&set1, &set2);
    println!("{:?}", difference)
}