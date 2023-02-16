fn main() {
    let mut s = String::from("Hello");
    s.push_str(", world!");
    println!("{}", s);

    let s1 = String::from("hello");
    let _s2 = s1; // s1 now becomes invalid and can no longer be used ...
    // s1 has been move to s2
    // Only happens for types that don't implement Copy. Those that do like literal types do not have this behavior.
    // println!("{_s1}");

    let s3 = String::from("hello");
    let s4 = s3.clone(); // a deep copy. It is expensive.
    println!("s3 = {s3}, s4 = {s4}");


    let s = String::from("hello");  // s comes into scope

    takes_ownership(s);             // s's value moves into the function...
    //println!("s after {}", s);     // ... and so is no longer valid here

    let x = 5;                      // x comes into scope

    makes_copy(x);                  // x would move into the function,
    println!("x after {}", x);      // but i32 is Copy, so it's okay to still
                                    // use x afterward 

    let s1 = String::from("hello");
    // s1 is destroyed but comes back as s2
    let (s2, len) = calculate_length(s1);

    println!("The length of '{}' is {}.", s2, len);
} // Here, x goes out of scope, then s. But because s's value was moved, nothing
  // special happens.

fn takes_ownership(some_string: String) { // some_string comes into scope
    println!("{}", some_string);
} // Here, some_string goes out of scope and `drop` is called. The backing
  // memory is freed.

fn makes_copy(mut some_integer: i32) { // some_integer comes into scope
    some_integer = some_integer + 1;
    println!("{}", some_integer);
} // Here, some_integer goes out of scope. Nothing special happens.

// we have to return the String to the calling function so we can still use the String after the call
fn calculate_length(s: String) -> (String, usize) {
    let length = s.len(); // len() returns the length of a String

    (s, length)
}
