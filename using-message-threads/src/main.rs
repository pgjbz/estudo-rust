use std::{sync::mpsc, thread, time::Duration};

fn main() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let val = String::from("pacoca");
        thread::sleep(Duration::from_secs(2));
        tx.send(val).unwrap();
        // println!("val if {}", val); borrow value
    });


    let received = rx.recv().unwrap(); //wait message send, block thread 
    println!("Got: {}", received);
    // if let Ok(received) = rx.try_recv() { //try_recv don't block the thread
    //     println!("God: {}", received);
    // } else {
    //     println!("Nothing");
    // }

    let (tx, rx) = mpsc::channel();

    let tx1 = tx.clone(); //using clone to produce more producers

    thread::spawn(move|| {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("Thread"),
            String::from("HAHA")
        ];
        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
        thread::sleep(Duration::from_secs(3));
    });

    thread::spawn(move|| {
        let vals = vec![
            String::from("hi1"),
            String::from("from1"),
            String::from("the1"),
            String::from("Thread1"),
            String::from("HAHA1")
        ];
        for val in vals {
            tx1.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
        thread::sleep(Duration::from_secs(3));
    });


    for receive in rx { //iter messages from threads, waiting for every message
        println!("Got: {}", receive);
    } //when the channel is closed, the iteration ill end.

}
