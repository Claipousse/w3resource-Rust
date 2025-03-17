use std::io;
use rand::Rng;

fn input_size() -> usize {
    loop {
        let mut size = String::new();
        println!("How many floats you want in the vector ?");
        io::stdin().read_line(&mut size).expect("Failed to read line");
        match size.trim().parse::<usize>() {
            Ok(n) => return n,
            Err(_) => println!("Error : Invalid input. Please try again.")
        }
    }
}

fn apply_closure<F>(vec:Vec<f64>, closure:F) -> f64
where
    F: FnMut(f64) -> f64
{
    let modified_vec:Vec<f64> = vec.into_iter()
        .map(closure)
        .map(|x| (x * 100.0).round() / 100.0)
        .collect();
    println!("Modified vector : {:?}", modified_vec);
    *modified_vec.iter().max_by(|a, b| a.partial_cmp(b).unwrap()).unwrap_or(&0.0)
}

fn main() {
    let mut rng = rand::rng();
    let size = input_size();
    let vec:Vec<f64> = (0..size)
        .map(|_| {
            let number:f64 = rng.random_range(0.0..=10.0);
            (number * 100.0).round() / 100.0
        })
        .collect();
    println!("{:?}", vec);

    let cube_closure = |x:f64| x * x * x;
    let maximum_modified_float = apply_closure(vec, cube_closure);
    println!("The maximum modified float of the vector is : {}", maximum_modified_float);
}
