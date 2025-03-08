use std::thread;
use std::sync::mpsc;
use std::io;

fn input_message() -> String {
    let mut input = String::new();
    println!("Input the message you want :");
    io::stdin().read_line(&mut input).expect("Error when reading the input...");
    let message = input.trim().to_string();
    message
}

fn main() {
    let (sender, receiver) = mpsc::channel();

    let thread1 = thread::spawn(move || {
        let message = input_message() ;
        sender.send(message).unwrap(); //Send the message
    });

    let thread2 = thread::spawn(move || {
        let received = receiver.recv().unwrap(); //Get the message we wrote
        println!("Message received : {received}")
    });
    thread1.join().unwrap();
    thread2.join().unwrap();
}