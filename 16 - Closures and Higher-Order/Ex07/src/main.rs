fn apply_closure<F>(range:std::ops::Range<i32>, closure:F) -> Vec<i32>
where
    F:FnMut(i32) -> i32
{
    range.into_iter().map(closure).collect()
}

fn main() {
    let range = 0..10;
    let square_numbers = apply_closure(range.clone(), |x:i32| x*x);
    println!("Range : {:?}",range);
    println!("Square range : {:?}", square_numbers)
}