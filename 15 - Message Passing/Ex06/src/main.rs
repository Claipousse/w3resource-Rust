use std::sync::mpsc;
use std::{io, thread};

fn input_number_of_messages() -> usize {
    loop {
        let mut input = String::new();
        println!("How many messages do you want to print?");
        io::stdin().read_line(&mut input).expect("Error when reading the input");
        match input.trim().parse::<usize>() {
            Ok(n) => return n,
            Err(_) => println!("Error : Input invalid. Please try again.")
        }
    }
}


fn main() {
    let number_of_messages = input_number_of_messages();
    let (sender, receiver) = mpsc::sync_channel(0);

    let sender_thread = thread::spawn(move || {
        for i in 0..number_of_messages {
            let message = format!("Message n°{}", i+1);
            println!("Sending : {message}");
            sender.send(message).unwrap();
        }
    });

    let receiver_thread = thread::spawn(move || {
        for _ in 0..number_of_messages {
            let received = receiver.recv().unwrap();
            println!("Message received : {}", received)
        }
    });

    sender_thread.join().unwrap();
    receiver_thread.join().unwrap()
}
