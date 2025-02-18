use std::io;

struct Rectangle {
    longueur: f64,
    largeur: f64
}

fn saisir_dimensions() -> Rectangle {
    let mut long = String::new();
    let mut larg = String::new();

    println!("Entrez la longueur : ");
    io::stdin().read_line(&mut long).expect("Erreur de lecture");
    let longueur: f64 = long.trim().parse().expect("Veuillez entrer un nombre valide");

    println!("Entrez la largeur : ");
    io::stdin().read_line(&mut larg).expect("Erreur de lecture");
    let largeur: f64 = larg.trim().parse().expect("Veuillez entrer un nombre valide");

    Rectangle{longueur, largeur}
}

impl Rectangle {
    fn aire(&self) -> f64 {
        self.longueur * self.largeur
    }
}

fn affichage(rectangle: Rectangle) {
    println!("Aire = {} * {} = {}", rectangle.longueur, rectangle.largeur, rectangle.aire())
}

fn main() {
    let rectangle:Rectangle = saisir_dimensions();
    affichage(rectangle)
}