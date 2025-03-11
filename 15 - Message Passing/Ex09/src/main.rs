use std::sync::{mpsc, Arc, Mutex};
use std::thread;

fn main() {
    let (request_sender, request_receiver) = mpsc::channel();
    let (response_sender, response_receiver) = mpsc::channel();
    let response_receiver = Arc::new(Mutex::new(response_receiver));

    let request_handler = thread::spawn(move || {
        for request in request_receiver {
            let processed_request = format!("Processed: {}", request);
            response_sender.send(processed_request).unwrap();
        }
    });

    let cloned_response_receiver = Arc::clone(&response_receiver);
    let response_handler = thread::spawn(move || {
        let receiver = cloned_response_receiver.lock().unwrap();
        for response in receiver.iter() {
            println!("Received response: {}", response);
        }
    });

    for i in 0..5 {
        request_sender.send(i.to_string()).unwrap();
        let response = response_receiver.lock().unwrap().recv().unwrap();
        println!("Received response to request {}: {}", i, response);
    }

    request_handler.join().unwrap();
    response_handler.join().unwrap();
}
