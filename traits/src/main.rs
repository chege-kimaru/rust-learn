use std::fmt::{Debug, Display};

use traits::{Summary, Tweet}; // must bring both struct and trait to scope

fn main() {
    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    };

    println!("1 new tweet: {}", tweet.summarize());

    notify(&tweet);
}

// trait as parameters
fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}

// trait bound syntax
fn notify2<T: Summary>(item: &T) {}

// Both item 1 and 2 have to be of same type eg Tweet only unlike notify4 below
fn notify3<T: Summary>(item1: &T, item2: &T) {}

// Item 1 and item 2 can be of different types as long as they impl Summary
fn notify4(item1: &impl Summary, item2: &impl Summary) {}

// Specifying Multiple Trait Bounds with the + Syntax
fn notify5(item: &(impl Summary + Display)) {}
// or
fn notify6<T: Summary + Display>(item: &T) {}

// clearer trait bounds with where clauses
fn some_function<T: Display + Clone, U: Clone + Debug>(t: &T, u: &U) -> i32 {
    8
}
//cleaner as
fn some_function2<T, U>(t: &T, u: &U) -> i32
where
    T: Display + Clone,
    U: Clone + Debug,
{
    8
}

// Returning types that implement Traits
fn returns_summarizable() -> impl Summary {
    Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    }
}
// Note: However, you can only use impl Trait if you’re returning a single type
// So this won't work
// fn returns_summarizable(switch: bool) -> impl Summary {
//     if switch {
//         NewsArticle {
//             headline: String::from("Penguins win the Stanley Cup Championship!"),
//             location: String::from("Pittsburgh, PA, USA"),
//             author: String::from("Iceburgh"),
//             content: String::from(
//                 "The Pittsburgh Penguins once again are the best \
//                  hockey team in the NHL.",
//             ),
//         }
//     } else {
//         Tweet {
//             username: String::from("horse_ebooks"),
//             content: String::from("of course, as you probably already know, people"),
//             reply: false,
//             retweet: false,
//         }
//     }
// }

// Using Trait Bounds to Conditionally Implement Methods
struct Pair<T> {
    x: T,
    y: T,
}

impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}
// only implements the cmp_display method if its inner type T implements the PartialOrd and Display traits
impl<T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest member is x = {}", self.x);
        } else {
            println!("The largest member is y = {}", self.y);
        }
    }
}

// TODO: Try understand this
// We can also conditionally implement a trait for any type that implements another trait.
// Implementations of a trait on any type that satisfies the trait bounds are called blanket implementations
// and are extensively used in the Rust standard library
// For example, the standard library implements the ToString trait on any type that implements the Display trait.
// The impl block in the standard library looks similar to this code:
// impl<T: Display> ToString for T {
//     // --snip--
// }
