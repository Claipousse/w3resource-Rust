use std::sync::{mpsc, Arc, Mutex};
use std::{io, thread};
use std::time::Duration;

const NUMBER_OF_CONSUMERS:usize = 3;

fn input_number_of_messages() -> usize {
    loop {
        let mut input = String::new();
        println!("How many messages you want to produce ?");
        io::stdin().read_line(&mut input).expect("Error : Failed to read line");
        match input.trim().parse::<usize>() {
            Ok(n) => return n,
            Err(_) => println!("Error : Invalid Input, please try again."),
        }
    }
}

fn main() {
    let (sender, receiver) = mpsc::channel();
    let receiver = Arc::new(Mutex::new(receiver));

    let number_of_messages = input_number_of_messages();
    let producer = thread::spawn(move || {
       for i in 1..=number_of_messages {
           println!("Message sent : {i}");
           sender.send(i).unwrap();
           thread::sleep(Duration::from_millis(300));
       }
    });

    let mut consumers = Vec::new();
    for id in 0..NUMBER_OF_CONSUMERS {
        let receiver_clone = receiver.clone();
        let consumer = thread::spawn(move || {
           while let Ok(data) = receiver_clone.lock().unwrap().recv() {
                println!("Consumer {} received : {}", id + 1 , data);
               thread::sleep(Duration::from_millis(300));
            }
        });
        consumers.push(consumer);
    }
    producer.join().unwrap();
    for consumer in consumers {
        consumer.join().unwrap();
    }
}