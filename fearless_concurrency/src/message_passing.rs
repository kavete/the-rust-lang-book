use std::sync::mpsc;
use std::thread;
use std::time::Duration;

pub fn msg_passing() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        // let val = String::from("hi");

        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("channel"),
        ];

        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_millis(1));
        }
    });

    // let received = rx.recv().unwrap();
    for received in rx {
        println!("Got {received}");
    }
}

//Mutiple producers
//

pub fn multi_producer() {
    let (tx, rx) = mpsc::channel();

    let tx1 = tx.clone();

    thread::spawn(move || {
        let vals = vec![String::from("Yoh Tx1"), String::from("again Tx1")];

        for val in vals {
            tx1.send(val).unwrap();
            thread::sleep(Duration::from_millis(1));
        }
    });

    thread::spawn(move || {
        let vals = vec![String::from("More from Tx"), String::from("more tx")];

        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_millis(1));
        }
    });

    for received in rx {
        println!("Got {received} ");
    }
}
