use std::collections::HashSet;
fn difference(set1: &HashSet<i32>, set2: &HashSet<i32>) -> HashSet<i32> {
    let mut difference_set = HashSet::new();

    for &element in set1 {
        if !set2.contains(&element) {
            difference_set.insert(element);
        }
    }
    difference_set
}

fn main() {
    let set1: HashSet<i32> = vec![1, 2, 3].into_iter().collect();
    let set2: HashSet<i32> = vec![3, 4, 5].into_iter().collect();

    let diff = difference(&set1, &set2);

    println!("{:?}", diff);
}