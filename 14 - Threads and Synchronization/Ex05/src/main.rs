use std::net::{TcpListener, TcpStream}; //Gestion des ports
use std::io::{Read, Write}; //Lecture et écritures des données transmises sur le port

fn handle_client(mut stream:TcpStream) {
    let mut buffer = [0; 512];
    stream.read(&mut buffer).unwrap();
    let request = String::from_utf8_lossy(&buffer[..]); //.. = prend tout le buffer
    println!("Requête reçue : {request}");
    let reponse:&str = "HTTP/1.1 200 OK\r\n\r\nLa connexion avec le serveur est réussi";
    stream.write(reponse.as_bytes()).unwrap(); //Ecrire reponse dans le flux client
    stream.flush().unwrap(); //On s'assure que toutes les données ont été envoyés
}

fn main() {
    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();
    println!("Le serveur est sur le port 8080...");

    // Accepter une seule connexion
    if let Ok((stream, _addr)) = listener.accept() {
        handle_client(stream);
    }
    println!("Le serveur se coupe.");
}
//Pour vérifier le code, lancez le programme, puis allez sur le CMD et tapez "curl http://127.0.0.1:8080/