use std::io;
use rand::Rng;

fn input_size() -> usize {
    loop {
        let mut input = String::new();
        println!("How many arrays you want in your vector ?");
        io::stdin().read_line(&mut input).expect("Error when reading the line...");
        match input.trim().parse::<usize>() {
            Ok(n) => return n,
            Err(_) => println!("Error : Invalid input, Please try again")
        }
    }
}

fn apply_closure<T, U, F>(vec:Vec<[T; 4]>, mut closure: F) -> Vec<[U; 4]>
where
    T: Copy,
    F: FnMut(T) -> U,
{
    vec.into_iter().map(|arr| [closure(arr[0]), closure(arr[1]), closure(arr[2]), closure(arr[3])]).collect()
}


fn main() {
    let mut rng = rand::thread_rng();
    let size = input_size();
    let arrays: Vec<[i32; 4]> = (0..size)
        .map(|_| {
            [
                rng.gen_range(0..101),
                rng.gen_range(0..101),
                rng.gen_range(0..101),
                rng.gen_range(0..101),
            ]
        })
        .collect();
    println!("Arrays: {:?}", arrays);

    let modified_arrays = apply_closure(arrays, |x| x * 2);
    println!("Modified arrays (x2) : {:?}", modified_arrays)
}
