use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn main() {
    // mpsc stands for: multiple producer, single consumer (imagine multiple streams flowing into one big river)
    // channel() returns a tuple with transmitter and receiver
    let (tx, rx) = mpsc::channel();

    // // We are using move to move tx into the closure so the spawned thread owns it
    // thread::spawn(move || {
    //     let val = String::from("hi");
    //     tx.send(val).unwrap();
    //     // println!("val is {}", val); // This will cause an error. The send function takes ownership of its parameter, and when the value is moved, the receiver takes ownership of it. This stops us from accidentally using the value again after sending it to prevent inconsistencies.
    // });

    // // recv blocks the main thread’s execution and wait until a value is sent down the channel
    // // try_recv method doesn’t block, but will instead return a Result<T, E> immediately: an Ok value holding a message if one is available and an Err value if there aren’t any messages this time. Useful if the thread has other work to do while waiting
    // let received = rx.recv().unwrap();
    // println!("Got: {}", received);

    // To use more than one transmitter, clone the transmitter
    let tx1 = tx.clone();
    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];

        // Sending Multiple Values and Seeing the Receiver Waiting
        for val in vals {
            tx1.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });


    thread::spawn(move || {
        let vals = vec![
            String::from("more"),
            String::from("messages"),
            String::from("for"),
            String::from("you"),
        ];

        // Sending Multiple Values and Seeing the Receiver Waiting
        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    // rx is treated as an iterator. For each value received. When the channel is closed, iteration will end.
    for received in rx {
        println!("Got: {}", received);
    }
}
