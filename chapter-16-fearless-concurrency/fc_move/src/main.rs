use std::thread;

fn main() {
    let v = vec![1, 2, 3];

    // Get an error since v is borrowed into the closure. v might be dropped before thread execution is complete.
    // Therefore v has to be moved into the closure using move keyword so as to take ownership of v
    let handle = thread::spawn(move || {
        println!("Here's a vector: {:?}", v);
    });

    // drop(v); // Error: value used after move

    handle.join().unwrap();
}
