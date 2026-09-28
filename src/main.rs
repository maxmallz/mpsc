use std::{println, sync::mpsc, thread, time::Duration};

fn main() {
    println!("Hello, world!");

    let (tx, rx) = mpsc::channel();

    let tx2 = tx.clone();

    thread::spawn(move || {
        let messages = vec![
            String::from("hello"),
            String::from("from"),
            String::from("over"),
            String::from("here"),
        ];

        for message in messages {
            tx.send(message).unwrap();
            thread::sleep(Duration::from_millis(600));
        }
    });

        thread::spawn(move || {
        let messages = vec![
            String::from("some"),
            String::from("more"),
            String::from("messages"),
            String::from("incoming"),
        ];

        for message in messages {
            tx2.send(message).unwrap();
            thread::sleep(Duration::from_millis(600));
        }
    });

    for message in rx {
        println!("Received {}", message);
    }
}
