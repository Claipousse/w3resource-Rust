use std::sync::{Arc, Mutex, MutexGuard};
use std::thread;

struct SharedQueue {
    data: Arc<Mutex<Vec<i32>>>, //Vecteur sécurisé, plusieurs threads peuvent y accéder
}

impl SharedQueue {
    fn new() -> Self {
        SharedQueue { //Mutex protège la donnée
            data: Arc::new(Mutex::new(Vec::new())),
        }
    }

    fn push(&self, valeur:i32) {
        let mut data:MutexGuard<Vec<i32>> = self.data.lock().unwrap();
        data.push(valeur);
        //lock() verrouille la donnée avant de l'utiliser
        //La donnée est pushé vers le vecteur
        //unwrap() garantit le verrouillage
    }

    fn pop(&self) -> Option<i32> {
        let mut data = self.data.lock().unwrap();
        data.pop() //Supprime et retourne le dernier élément
    }
}

fn main() {
    //Création file partagée entre différents threads, protégée par un Mutex
    let shared_queue = Arc::new(SharedQueue::new());

    //Donne les reférences aux threads producer et consumer
    let producer_queue = Arc::clone(&shared_queue);
    let consumer_queue = Arc::clone(&shared_queue);

    let producer_handle = thread::spawn(move || {
        for i in 0..5 {
            producer_queue.push(i);
            println!("Producteur a envoyé : {}", i);
            thread::sleep(std::time::Duration::from_millis(100));
        }
    });

    let consumer_handle = thread::spawn(move || {
        loop {
            if let Some(valeur) = consumer_queue.pop() {
                println!("Le consommateur a reçu : {}", valeur)
            } else {
                break; //Si file vide, on arrête
            }
            thread::sleep(std::time::Duration::from_millis(200));
        }
    });

    producer_handle.join().unwrap();
    consumer_handle.join().unwrap();
}