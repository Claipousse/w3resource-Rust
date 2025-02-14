use std::io;

fn saisie_celsius() -> f32 {
    let mut celsius = String::new();
    loop {
        println!("Saisissez une température (en °C)");
        io::stdin().read_line(&mut celsius).expect("Erreur de lecture...");
        match celsius.trim().parse::<f32>() {
            Ok(n) => {
                return n
            }
            Err(_) => {
                println!("Veuillez saisir une température !");
                continue
            }
        }
    }
}

fn conversion_fahrenheit(celsius:f32) -> f32 {
    celsius + 273.15
}

fn main() {
    let celsius:f32 = saisie_celsius();
    println!("{}°C = {} K",celsius, conversion_fahrenheit(celsius))
}
