use std::{println, sync::mpsc, thread};

fn main() {
    println!("Hello, world!");

    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let msg = String::from("hi");
        println!("Sent {}", msg);
        tx.send(msg).unwrap();
    });

    let msg = rx.recv().unwrap();
    println!("Received {}", msg);
}
