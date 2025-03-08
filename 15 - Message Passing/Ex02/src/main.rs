use std::{io, thread};
use std::sync::mpsc;
use std::time::Duration;

fn input_string() -> Vec<String> {
    let mut vec:Vec<String> = Vec::new();
    println!("Input strings (enter 'exit' to leave) :");
    loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Error when reading the input...");
        let input = input.trim().to_string();

        if input.eq_ignore_ascii_case("exit") {
            return vec
        }
        vec.push(input)
    }
}

fn main() {
    let vec:Vec<String> = input_string();

    let (sender, receiver) =mpsc::channel();

    let sender_thread = thread::spawn(move || {
        for messages in vec {
            println!("{messages} sent !");
            sender.send(messages).unwrap();
            thread::sleep(Duration::from_millis(700))
        }
    });
    let receiver_thread = thread::spawn(move || {
        for received in receiver {
            println!("{received} Received")
        }
    });
    sender_thread.join().unwrap();
    receiver_thread.join().unwrap();
}