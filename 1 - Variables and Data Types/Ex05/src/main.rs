use std::io;

fn saisie_temperature() -> f32 {
    loop {
        let mut temperature = String::new();
        println!("Saisissez la température (en °C) :");
        io::stdin().read_line(&mut temperature).expect("Erreur de lecture...");
        match temperature.trim().parse::<f32>() {
            Ok(n) => return n,
            Err(_) => println!("Veuillez saisir une température !")
        }
    }
}

fn conversion_fahrenheit(temperature:f32) -> f32{
    (temperature * (9.0 / 5.0)) + 32.0
}

fn affichage(celsius:f32, fahrenheit:f32) {
    println!("Température en Celsius : {} °C", celsius);
    println!("Température en Fahrenheit : {} F", fahrenheit)
}

fn main() {
    let celsius:f32 = saisie_temperature();
    let fahrenheit:f32 = conversion_fahrenheit(celsius);
    affichage(celsius, fahrenheit)
}