use std::{thread, time::Duration};

fn main() {
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    // Calling join on the handle blocks the thread currently running until the thread represented by the handle terminates. Blocking a thread means that thread is prevented from performing work or exiting. Main thread will be blocked in this case.
    // handle.join().unwrap();

    for i in 1..5 {
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }

    // Note that when the main thread of a Rust program completes, all spawned threads are shut down, whether or not they have finished running.
    // join(), called on JoinedHandle returned by thread::spawn will wait for the thread to finish before main exists
    handle.join().unwrap();
}
