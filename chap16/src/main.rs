use std::thread;
use std::time::Duration;
use std::sync::mpsc;


fn main() {
    let v = vec![1, 2, 3];
    let handle = thread::spawn(move || {
        for i in 1..10 {
            println!("Hi number {} from the spawned thread!", i);
            println!("vector v = {:?}", v);
            thread::sleep(Duration::from_millis(1));
        }    
    });
    
    for i in 1..5 {
        println!("Hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }

    handle.join().unwrap();


    let (tx, rx) = mpsc::channel();
    thread::spawn(move || {
        let values = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];
        for val in values {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    for received in rx {
        println!("Got: {}", received);
    }
    
}
