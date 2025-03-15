fn apply_closure<F>(vec1: &Vec<bool>, vec2: &Vec<bool>, closure:F) -> Vec<bool>
where
    F:Fn(bool, bool) -> bool
{
    vec1.iter()
        .zip(vec2.iter())
        .map(|(&a, &b)| closure(a, b))
        .collect()
}

fn main() {
    let vec1:Vec<bool> = vec![true, false, true, false];
    let vec2:Vec<bool> = vec![true, true, false, false];
    let and = apply_closure(&vec1, &vec2, |a:bool, b:bool| a && b);
    for i in 0..=3 {
        println!("{} * {} -> {}", vec1[i], vec2[i], and[i])
    }
}
