use std::thread;

fn main() {
    let thread1 = thread::spawn(|| {
        println!("Hello World !")
    });
    let thread2= thread::spawn(|| {
        println!("Hello World !")
    });
    thread1.join().unwrap();
    thread2.join().unwrap()
}
