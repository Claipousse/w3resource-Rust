use std::collections::HashSet;

fn trouver_intersection<'a>(rouge:&HashSet<&'a str>, fruits:&HashSet<&'a str>) -> HashSet<&'a str> {
    let mut intersection = HashSet::new();
    for &elements in rouge {
        if fruits.contains(elements) {
            intersection.insert(elements);
        }
    }
    intersection
}

fn main() {
    let rouge: HashSet<&str> = ["Fraise", "Cocinelle", "Robe"].iter().cloned().collect();
    let fruits: HashSet<&str> = ["Mangue", "Pastèque", "Fraise"].iter().cloned().collect();
    let intersection = trouver_intersection(&rouge, &fruits);
    println!("Les éléments en commun dans les deux sets sont : {:?}", intersection)
}