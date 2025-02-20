use std::io;

#[derive(Debug)]
struct Point {
    x: f64,
    y:f64
}

fn saisie_coordonnees(message_x:&str, message_y:&str) -> Point{
    let mut x = String::new();
    let mut y = String::new();
    println!("{}", message_x);
    io::stdin().read_line(&mut x).expect("Erreur de lecture...");
    let x:f64 = x.trim().parse().expect("Saisissez une coordonnée correcte !");
    println!("{}", message_y);
    io::stdin().read_line(&mut y).expect("Erreur de lecture...");
    let y:f64 = y.trim().parse().expect("Saisissez une coordonnées correcte !");
    Point{x, y}
}

impl Point {
    fn distance(&self, other: &Point) -> f64{
        ((self.x - other.x).powi(2) + (self.y - other.y).powi(2)).sqrt()
    }
}

fn affichage(p1:Point, p2:Point, distance:f64) {
    println!("La distance entre {:?} et {:?} est : {}", p1, p2, distance)
}

fn main() {
    let p1: Point = saisie_coordonnees("Saisissez X du point 1 :", "Saisissez Y du point 1 :");
    let p2: Point = saisie_coordonnees("Saisissez X du point 2 :", "Saisissez Y du point 2 :");
    let distance = p1.distance(&p2);
    affichage(p1, p2, distance)
}