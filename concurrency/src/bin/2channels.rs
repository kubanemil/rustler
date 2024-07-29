/*
Data is sent between threads via - channels. Channel has 2 halves: transmitter and receiver. 
Channel is closed if either transmitter or receiver half is dropped.
tx - transmitter
rx - receiver
*/
use std::{sync::mpsc, thread, time::Duration}; // mutliple producer, single-consumer

fn main() {
    let (tx, rx) = mpsc::channel::<&str>();

    thread::spawn(move || {
        let messages = vec!["Hello", "guys", "!"];
        for message in messages {
            tx.send(message).unwrap(); // will panic if receiver is dropped.
            println!("sent_message: {message}");
            thread::sleep(Duration::from_millis(600));
        }
    });

    // .recv() will block main thread and wait until it receives message
    // .try_recv() won't block main thread, will return Err() if no any message.
    let received_messages = rx.recv().unwrap(); // will panic if transmitter is dropped
    println!("Received_message: {:?}\n", received_messages);

    for received in rx { // as iterator, will wait for next message
        println!("received_message: {}\n", received);
    }

    println!("Multiple transmitters:");
    multi_transmitter();
}

fn multi_transmitter() {
    let (tx, rx) = mpsc::channel::<(&str, u8)>();
    let tx2 = tx.clone(); // clone transmitter to use in other thread

    thread::spawn(move || {        
        for msg in [11, 22, 33] {
            tx.send(("tx", msg)).unwrap();
            thread::sleep(Duration::from_millis(300));
        }
    });

    thread::spawn(move || {
        for msg in [1, 2, 3] {
            tx2.send(("tx2", msg)).unwrap();
            thread::sleep(Duration::from_millis(300));
        }
    });

    for msg in rx {
        println!("Got: ({}, {})", msg.0, msg.1);
    }
}