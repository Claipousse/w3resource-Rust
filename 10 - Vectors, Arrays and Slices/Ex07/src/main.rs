fn main() {
    let jours:[&str; 6] = ["Lundi", "Mardi", "Mercredi", "Jeudi", "Vendredi", "Samedi"];
    let jours_slice = &jours[0..=2];
    println!("{:?}", jours_slice)
}