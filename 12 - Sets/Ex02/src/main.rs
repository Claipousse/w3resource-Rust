use std::collections::HashSet;

fn trouver_union<'a>(rouge:&HashSet<&'a str>, fruits:&HashSet<&'a str>) -> HashSet<&'a str> {
    let mut union = rouge.clone();
    for &element in fruits {
        union.insert(element);
    }
    union
}

fn main() {
    let rouge: HashSet<&str> = ["Fraise", "Cocinelle", "Robe"].iter().cloned().collect();
    let fruits: HashSet<&str> = ["Mangue", "Pastèque", "Fraise"].iter().cloned().collect();
    let union:HashSet<&str> = trouver_union(&rouge, &fruits);
    println!("Les éléments des deux sets sont : {:?}", union)
}