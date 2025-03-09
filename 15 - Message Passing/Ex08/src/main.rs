use std::io;
use std::thread;
use std::sync::mpsc;

fn input_messages() -> Vec<String> {
    let mut vec = Vec::new();
    println!("Enter at least five messages ('exit') :");
    loop {
        let mut message = String::new();
        io::stdin().read_line(&mut message).expect("Error when reading the input...");
        let message = message.trim().to_string();
        let clone_message = message.clone().to_lowercase();

        if clone_message == "exit" && vec.len() >= 5 {
            return vec
        }
        if clone_message == "exit" && vec.len() < 5 {
            println!("Error, You need to enter at least {} more messages.", 5 - vec.len())
        }
        else {
            vec.push(message)
        }
    }
}


fn main() {
    let messages:Vec<String> = input_messages();
    let number_of_messages:usize = messages.len();
    let (sender, receiver) = mpsc::sync_channel(2);

    let sender_thread = thread::spawn(move || {
        for message in messages {
            println!("Sent : {message}");
            sender.send(message).expect("Failed to send message")
        }
    });

    let receiver_thread = thread::spawn(move || {
        for _ in 0..number_of_messages {
            let received = receiver.recv().unwrap();
            println!("Message received : {}", received)
        }
    });

    sender_thread.join().unwrap();
    receiver_thread.join().unwrap();
}
