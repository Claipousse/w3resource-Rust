use chrono::{Datelike, Local};
use std::io;

fn saisie_date_naissance() -> Result<(u8, u8, u16), String> {
    loop {
        let mut input = String::new();
        println!("Saisissez votre date de naissance (format DD/MM/YYYY) :");
        io::stdin().read_line(&mut input).expect("Erreur de lecture...");
        let input = input.trim();
        let parts: Vec<&str> = input.split('/').collect();

        if parts.len() != 3 {
            println!("Format invalide ! Utilisez le format DD/MM/YYYY.");
            continue;
        }
        let jour_naissance: u8 = match parts[0].parse() {
            Ok(j) if j > 0 && j <= 31 => j,
            _ => {
                println!("Jour invalide !");
                continue;
            }
        };
        let mois_naissance: u8 = match parts[1].parse() {
            Ok(m) if m > 0 && m <= 12 => m,
            _ => {
                println!("Mois invalide !");
                continue;
            }
        };
        let annee_naissance: u16 = match parts[2].parse() {
            Ok(a) if a > 1900 && a <= Local::now().year() as u16 => a,
            _ => {
                println!("Année invalide !");
                continue;
            }
        };
        return Ok((jour_naissance, mois_naissance, annee_naissance));
    }
}

fn difference_date(jour:u8, mois:u8, annee:u16) -> (u16, u8, u8) {
    let aujourdhui = Local::now();
    let mut jour_ajd = aujourdhui.day() as u8;
    let mut mois_ajd = aujourdhui.month() as u8;
    let mut annee_ajd = aujourdhui.year() as u16;

    if jour_ajd < jour {
        mois_ajd -= 1;
        jour_ajd += 30; // Approximation, ne tient pas compte des mois exacts
    }

    let jour_diff = jour_ajd - jour;

    if mois_ajd < mois {
        annee_ajd -= 1;
        mois_ajd += 12;
    }

    let mois_diff = mois_ajd - mois;
    let annee_diff = annee_ajd - annee;

    (annee_diff, mois_diff, jour_diff)
}

fn main() {
    match saisie_date_naissance() {
        Ok((jour, mois, annee)) => {
            let (annees, mois, jours) = difference_date(jour, mois, annee);
            println!("Votre âge : {} ans, {} mois et {} jours.", annees, mois, jours);
        }
        Err(err) => println!("Erreur : {}", err),
    }
}