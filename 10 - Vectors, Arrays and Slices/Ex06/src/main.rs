fn main() {
    let tableau:[i32; 8] = [1, 2, 3, 4, 5, 6, 7, 8];
    let tableau_slice = &tableau[2..=5];
    println!("Tableau : {:?}", tableau);
    println!("Tableau slicé : {:?}", tableau_slice)
}