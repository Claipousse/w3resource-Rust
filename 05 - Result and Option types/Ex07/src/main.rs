fn hex_string_to_int(hex_string: &str) -> Option<u64> {
    match u64::from_str_radix(hex_string, 16) {
        Ok(nombre) => Some(nombre),
        Err(_) => None,
    }
}

fn main() {
    let hex_valide = "BF3C";
    let hex_invalide = "IJKL";

    match hex_string_to_int(hex_valide) {
        Some(nombre) => println!("Nombre converti : {}", nombre),
        None => println!("Entrée hexadécimale invalide"),
    }

    match hex_string_to_int(hex_invalide) {
        Some(nombre) => println!("Nombre converti : {}", nombre),
        None => println!("Entrée hexadécimale invalide"),
    }
}
