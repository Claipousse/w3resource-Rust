#[derive(Debug)]
enum Couleur {
    Rouge,
    Vert,
    Bleu,
}

fn main() {
    let rouge = Couleur::Rouge;
    let vert = Couleur::Vert;
    let bleu = Couleur::Bleu;

    println!("Rouge: {:?}", rouge);
    println!("Vert: {:?}", vert);
    println!("Bleu: {:?}", bleu);
}