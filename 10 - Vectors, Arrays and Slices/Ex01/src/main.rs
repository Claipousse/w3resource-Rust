fn main() {
    let vecteur1:Vec<i32> = (1..=10).collect();
    let vecteur2 = &vecteur1[3..=7];
    println!("Vecteur initial : {:?}", vecteur1);
    println!("Vecteur slicé : {:?}", vecteur2)
}