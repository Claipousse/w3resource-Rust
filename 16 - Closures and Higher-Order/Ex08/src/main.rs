fn sum_with_closure<F> (slice:&[i32], closure:F) -> i32
where F: Fn(i32) -> i32
{
    slice.iter().map(|&x| closure(x)).sum()
}

fn main() {
    let numbers = [1, 2, 3, 4, 5];
    let sum = sum_with_closure(&numbers, |x:i32| x * x);
    println!("{}", sum);
}
