use rand::Rng;

fn main() {
    let mut rng = rand::rng();

    let tab:[i32; 9] = [(); 9].map(|_| rng.random_range(0..=10));
    let tab_cube: Vec<i32> = tab.iter().cloned().map(|x| x*x*x).collect();
    let tab_slice = &tab_cube[1..=6];

    println!("Tableau : {:?}", tab);
    println!("Tableau au cube : {:?}", tab_cube);
    println!("Tableau slicé : {:?}", tab_slice)
}
