fn main() {
    // let mut s = String::from("hello world");
    let s = String::from("hello world");

    // start index and last index + 1 (length)
    let hello = &s[0..5];
    let _world = &s[6..11];

    println!("{hello}");

    let _slice = &s[0..2];
    let _slice = &s[..2];

    let len = s.len();
    let _slice = &s[3..len];
    let _slice = &s[3..];

    let _slice = &s[0..len];
    let _slice = &s[..];

    let word = first_word2(&s); // immutable borrow occurs here
    // s.clear(); // error mutable borrow occurs here to truncate the string. immutable + mutable XX
    println!("First word is: {word}"); 

    // let s = "Hello world"; // string literals are slices ie &s

    first_word2("hello");
    first_word2(&String::from("Hello"));

    let a = [1, 2, 3, 4 ,5];
    let slice = &a[1..3]; // slice of &i32
    assert_eq!(slice, &[2, 3]); 

}

// A slice is a kind of reference, so it does not have ownership.

fn _first_word(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }
    s.len()
}

// fn first_word2(s: &String) -> &str {
// String references are whole slices of a string so will work here eg first_word2(&String::from("Hello"))
// String literals are slices so will also work eg first_word2("Hello")
fn first_word2(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}