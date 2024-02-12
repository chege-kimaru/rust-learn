use core::prelude::v1;
use std::{thread, time::Duration};

#[derive(Debug, PartialEq, Copy, Clone)]
enum ShirtColor {
    Red,
    Blue,
}

struct Inventory {
    shirts: Vec<ShirtColor>,
}

impl Inventory {
    fn giveaway(&self, user_preference: Option<ShirtColor>) -> ShirtColor {
        user_preference.unwrap_or_else(|| self.most_stocked())
    }

    fn most_stocked(&self) -> ShirtColor {
        let mut num_red = 0;
        let mut num_blue = 0;

        for shirt in &self.shirts {
            match shirt {
                ShirtColor::Red => num_red += 1,
                ShirtColor::Blue => num_blue += 1,
            }
        }

        if num_red > num_blue {
            ShirtColor::Red
        } else {
            ShirtColor::Blue
        }
    }
}

fn main() {
    // === Capturing the Environment with Closures ===

    let store = Inventory {
        shirts: vec![ShirtColor::Blue, ShirtColor::Red, ShirtColor::Blue],
    };

    let user_pref1 = Some(ShirtColor::Red);
    let giveaway1 = store.giveaway(user_pref1);
    println!(
        "The user with preference {:?} gets {:?}",
        user_pref1, giveaway1
    );

    let user_pref2 = None;
    let giveaway2 = store.giveaway(user_pref2);
    println!(
        "The user with preference {:?} gets {:?}",
        user_pref2, giveaway2
    );

    // === Closure Type Inference and Annotation ===

    let expensive_closure = |num: u32| -> u32 {
        println!("Calculating slowly...");
        // thread::sleep(Duration::from_secs(2));
        num
    };
    expensive_closure(42);

    let add_one_v2 = |x: u32| -> u32 { x + 1 };
    // let add_one_v3  = |x| {x + 1};
    // let add_one_v4  = |x| x + 1;

    let example_closure = |x| x;
    let s = example_closure(String::from("hello"));
    // let n = example_closure(5); // Error: type of closure arg and return type have already been inferred as string

    // === Capturing References or Moving Ownership ===

    // Borrowing immutably
    let list = vec![1, 2, 3];
    println!("Before defining closure: {:?}", list);
    let only_borrows = || println!("from closure {:?}", list);
    println!("Before calling closure: {:?}", list);
    only_borrows();
    println!("After calling closure: {:?}", list);

    // Borrowing mutably
    let mut list = vec![1, 2, 3];
    println!("Before defining closure: {:?}", list);
    let mut borrows_mutably = || list.push(7);
    // println!("Before calling closure: {:?}", list); // Error: no other borrow is allowed when there's is a mutable borrow
    borrows_mutably(); // mutable borrow ends as we don't use the closure again
    println!("After calling closure: {:?}", list);

    // Moving ownership
    let list = vec![1, 2, 3];
    println!("Before defining closure: {:?}", list);
    thread::spawn(move || println!("From thread: {:?}", list))
        .join()
        .unwrap(); // list has to be moved as main thread can finish before the spawned thread

    // === Moving Captured Values Out of Closures and the Fn Traits ===
    // FnOnce applies to closures that can be called once. All closures implement at least this trait, because all closures can be called. A closure that moves captured values out of its body will only implement FnOnce and none of the other Fn traits, because it can only be called once.
    // FnMut applies to closures that don’t move captured values out of their body, but that might mutate the captured values. These closures can be called more than once.
    // Fn applies to closures that don’t move captured values out of their body and that don’t mutate captured values, as well as closures that capture nothing from their environment. These closures can be called more than once without mutating their environment, which is important in cases such as calling a closure multiple times concurrently.

    // === Iterators

    let v1 = vec![1, 2, 3];
    let v1_iter = v1.iter();
    for val in v1_iter {
        println!("Got {}", val)
    }
}

fn add_one_v1(x: u32) -> u32 {
    x + 1
}


#[test]
fn iterator_demonstartion() {
    let v1 = vec![1, 2, 3];

    let mut v1_iter = v1.iter(); // Note the mut as next method changes internal state of the iterator

    assert_eq!(v1_iter.next(), Some(&1));
    assert_eq!(v1_iter.next(), Some(&2));
    assert_eq!(v1_iter.next(), Some(&3));
    assert_eq!(v1_iter.next(), None);

    // values from next are immutable refrences to the values in the vector
    // To create an iterator that takes ownership of v1 and returns owned values, we can call into_iter instead of iter
    // Similarly, if we want to iterate over mutable references, we can call iter_mut instead of iter
}

// == Methods that Consume the Iterator : Consuming adaptors
#[test]
fn iterator_sum() {
    let v1 = vec![1, 2, 3];

    let v1_iter = v1.iter();

    let total: i32 = v1_iter.sum();

    assert_eq!(total, 6);

    // Note: We aren’t allowed to use v1_iter after the call to sum because sum takes ownership of the iterator we call it on
}

// === Methods that Produce Other Iterators : Iterator adaptors
#[test]
fn iterator_map() {
    let v1 = vec![1, 2, 3];

    let v1_iter = v1.iter();

    // v1_iter.map(|x| x + 1); // Warning: iterators are lazy and do nothing unless consumed. The closure is never called. So we use collect as below

    let v2: Vec<_> = v1_iter.map(|x| x + 1).collect();

    assert_eq!(v2, vec![2, 3, 4]);

}
