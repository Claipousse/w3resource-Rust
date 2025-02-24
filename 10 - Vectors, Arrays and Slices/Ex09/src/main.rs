use rand::Rng;

fn main() {
    let mut rng = rand::rng();

    let tab: [f64; 10] = std::array::from_fn(|_| {
        let nombre: f64 = rng.random_range(0.0..=1.0);
        (nombre * 10.0).round() / 10.0
    });
    let tab_filtre:Vec<f64> = tab.iter().copied().filter(|&x| x >= 0.5).collect();
    let tab_slice = if tab_filtre.len() >= 3 {
        &tab_filtre[0..3]
    } else {
        &tab_filtre
    };

    println!("Tableau : {:?}", tab);
    println!("Tableau filtré : {:?}", tab_filtre);
    if tab_slice.len() >= 3 {
        println!("Les trois premiers nombres sont : {:?}", tab_slice)
    }
}