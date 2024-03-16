use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    // Mutex is an abbreviation for mutual exclusion, as in, a mutex allows only one thread to access some data at any given time. To access the data in a mutex, a thread must first signal that it wants access by asking to acquire the mutex’s lock

    let m = Mutex::new(5);

    {
        // To access the data inside the mutex, we use the lock method to acquire the lock. This call will block the current thread so it can’t do any work until it’s our turn to have the lock.
        // The call to lock would fail if another thread holding the lock panicked.
        let mut num = m.lock().unwrap();
        *num = 6;
    }

    println!("m = {:?}", m);

    // We can't use just Mutex::new(0) since the counter will be owned by many threads
    // We can't use Rc<Mutex<i32>>, which allows multiple owners since Rc is not thread safe
    // So we use Arc - Atomic reference counter instead of Rc which is memory safe
    // Note,  just like using Rc<T> came with the risk of creating reference cycles, Mutex<T> comes with risk of deadlocks. These occur when an operation needs to lock two resources and two threads have each acquired one of the locks, causing them to wait for each other forever
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            // counter is immutable but we could get a mutable reference to the value inside it; this means Mutex<T> provides interior mutability, as the Cell family does. In the same way we used RefCell<T> in to allow us to mutate contents inside an Rc<T>, we use Mutex<T> to mutate contents inside an Arc<T>
            let mut num = counter.lock().unwrap();

            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap());

    // The Send marker trait indicates that ownership of values of the type implementing Send can be transferred between threads.
    // The Sync marker trait indicates that it is safe for the type implementing Sync to be referenced from multiple threads.
}
