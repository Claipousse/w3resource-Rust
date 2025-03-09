use std::sync::{mpsc, Arc, Mutex};
use std::thread;

const NUM_RECEIVERS:usize = 5;
const MESSAGE:&str = "Hello World!";

fn main() {
    let (sender, receiver) = mpsc::channel();
    let receiver = Arc::new(Mutex::new(receiver));

    let sender_handle = thread::spawn(move || {
        for _ in 0..NUM_RECEIVERS {
            sender.send(String::from(MESSAGE)).unwrap();
        }
        drop(sender) //Close the channel after sending all the messages
    });

    let mut receivers = Vec::new();
    for _ in 0..NUM_RECEIVERS {
        let rx = Arc::clone(&receiver);
        let receiver = thread::spawn(move || {
            loop {
                match rx.lock().unwrap().recv() {
                    Ok(msg) => println!("Received message : {msg}"),
                    Err(_) => break, //Break the loop if channel closed
                }
            }
        });
        receivers.push(receiver);
    }

    sender_handle.join().unwrap();
    for receiver in receivers {
        receiver.join().unwrap()
    }
}