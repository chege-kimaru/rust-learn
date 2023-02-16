fn main() {
    // Error: Dangling reference
    // let r;
    // {
    //     let x = 5;
    //     r = &x; // Error: `x` does not live long enough borrowed value does not live long enough
    // }
    // println!("r: {}", r);

    // // Correct: This works
    // let x = 5;
    // let r = &5;
    // println!("r: {}", r);

    // // This works with a function with lifetime annotations
    // let string1 = String::from("abcd");
    // let string2 = "xyz";
    // let result = longest(string1.as_str(), string2);
    // println!("The longest string is {}", result);

    // // This works as the result references sth that is valid until the end of the inner scope
    // let string1 = String::from("abcd");
    // {
    //     let string2 = String::from("xyz");
    //     let result = longest(string1.as_str(), string2);
    //     println!("The longest string is {}", result);
    // }

    // This won't work: We’ve told Rust that the lifetime of the reference returned by the
    // longest function is the same as the smaller of the lifetimes of the references passed in
    // in this case, result lives longer than string2
    // let string1 = String::from("abcd");
    // let result;
    // {
    //     let string2 = String::from("xyz");
    //     result = longest(string1.as_str(), string2.as_str());
    // }
    // println!("The longest string is {}", result);

    // This will work using longest2() as it only considers lifetime of the first parameter
    // let string1 = String::from("abcd");
    // let result;
    // {
    //     let string2 = String::from("xyz");
    //     result = longest2(string1.as_str(), string2.as_str());
    // }
    // println!("The longest string is {}", result);
}

// Error: this function's return type contains a borrowed value, but the signature does not say whether it is borrowed from `x` or `y`
// We don’t know the concrete lifetimes of the references that will be passed in
// The borrow checker doesn’t know how the lifetimes of x and y relate to the lifetime of the return value.
// fn longest(x: &str, y: &str) -> &str {
//     if x.len() > y.len() {
//         x
//     } else {
//         y
//     }
// }

// &i32        // a reference
// &'a i32     // a reference with an explicit lifetime
// &'a mut i32 // a mutable reference with an explicit lifetime

// The lifetime of the reference returned by the longest function is the same as the smaller of
// the lifetimes of the values referred to by the function arguments
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

// We’ve specified a lifetime parameter 'a for the parameter x and the return type, but not for the parameter y,
// because the lifetime of y does not have any relationship with the lifetime of x or the return value
fn longest2<'a>(x: &'a str, y: &str) -> &'a str {
    x
}

// Error
// When returning a reference from a function, the lifetime parameter for the return type needs to match the lifetime parameter for one of the parameters.
// If the reference returned does not refer to one of the parameters, it must refer to a value created within this function.
// However, this would be a dangling reference because the value will go out of scope at the end of the function
// fn longest3<'a>(x: &str, y: &str) -> &'a str {
//     let result = String::from("really long string!");
//     result.as_str()
// }

// // Lifetime annotations in struct definitions
struct ImportantExcerpt<'a> {
    part: &'a str,
}
impl<'a> ImportantExcerpt<'a> {
    // First ellison rule. See below.
    // So no need to annotate with lifetimes
    fn level(&self) -> i32 {
        3
    }

    // Third ellison rule
    // There are two input lifetimes, so Rust applies the first lifetime elision rule and gives both
    // &self and announcement their own lifetimes. Then, because one of the parameters is &self,
    // the return type gets the lifetime of &self, and all lifetimes have been accounted for.
    // So again, no need to annotate
    fn announce_and_return_part(&self, announcement: &str) -> &str {
        println!("Attention please: {}", announcement);
        self.part
    }
}

// fn main() {
//     let novel = String::from("Call me Ishmael. Some years ago...");
//     let first_sentence = novel.split('.').next().expect("Could not find a '.'");
//     let i = ImportantExcerpt {
//         part: first_sentence,
//     };
// }

// fn main() {
//     let novel = String::from("Call me Ishmael. Some years ago...");
//     let first_sentence = novel.split('.').next().expect("Could not find a '.'");
//     let i = ImportantExcerpt {
//         part: first_sentence,
//     };
// }

// Lifetime Elision
// No need for lifetimes
fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

/***
 * Lifetimeelision  rules:
 * 1. The first rule is that the compiler assigns a lifetime parameter to each parameter that’s a reference
 * In other words, a function with one parameter gets one lifetime parameter: fn foo<'a>(x: &'a i32);
 * a function with two parameters gets two separate lifetime parameters: fn foo<'a, 'b>(x: &'a i32, y: &'b i32); and so on.
 * 2. The second rule is that, if there is exactly one input lifetime parameter, that lifetime is assigned to all output lifetime parameters:
 * fn foo<'a>(x: &'a i32) -> &'a i32.
 * 3. The third rule is that, if there are multiple input lifetime parameters,
 * but one of them is &self or &mut self because this is a method,
 * the lifetime of self is assigned to all output lifetime parameters.
 */


 // Static lifetime
 // 'static, denotes that the affected reference can live for the entire duration of the program
 // Eg string literals have static lifetimes.

//  let s: &'static str = "I have a static liftime";

// Generic Type Parameters, Trait Bounds, and Lifetimes Together
use std::fmt::Display;
// lifetimes are a type of generics, so they go in <> together
// fn longest_with_an_announcement<'a, T>(
//     x: &'a str,
//     y: &'a str,
//     ann: T,
// ) -> &'a str
// where
//     T: Display,
// {
//     println!("Announcement! {}", ann);
//     if x.len() > y.len() {
//         x
//     } else {
//         y
//     }
// }