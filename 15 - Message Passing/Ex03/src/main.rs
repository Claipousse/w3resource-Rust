use std::sync::mpsc;
use std::thread;

const NUM_SENDERS:usize = 3;
const NUM_MESSAGES_PER_SENDERS:usize = 5;


fn main() {
    let (sender, receiver) = mpsc::channel();
    for i in 0..NUM_SENDERS {
        let sender_clone = sender.clone();
        thread::spawn(move || {
           for j in 0..NUM_MESSAGES_PER_SENDERS {
               let message = format!("Sender {} Message {} ", i+1, j+1);
               sender_clone.send(message).expect("Error sending message");
           }
        });
    }
    thread::spawn(move || {
        for received in receiver {
            println!("Received: {}", received);
        }
    });
    thread::sleep(std::time::Duration::from_secs(1));
}