use rand::Rng;

fn main() {
    let mut rng = rand::rng();
    let mut tab:[i32; 12] = [(); 12].map(|_| rng.random_range(0..=100));
    tab.sort();
    let tab_slice = &tab[tab.len() - 4..];
    println!("Tableau (filtré par ordre croissant) : {:?}", tab);
    println!("Les 4 plus grands nombres sont donc : {:?}", tab_slice)
}