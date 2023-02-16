fn main() {
    let s1 = String::from("hello");

    let len = calculate_length(&s1);

    println!("The length of '{}' is {}.", s1, len);

    change(&s1); // won't work

    let mut s = String::from("hello");
    change2(&mut s); // mutable reference
    println!("==== {s}");

    let mut s = String::from("hello");
    let r1 = &mut s;
    // let r2 = &mut s; // FAILS: if you have a mutable reference to a value, you can have no other references to that value
    // s.push_str("11"); // won't work
    r1.push_str("22");
    println!("{}", r1);
    s.push_str("11"); // this works
    println!("{}", s); 
    // println!("{}", r1); // won't work
    println!("{}", s); // works
    // from the above you notice that:  reference’s scope starts from where 
    // it is introduced and continues through the last time that reference is used.

    let mut st = String::from("hello!");
    let rt1 = &st; // no prob
    let rt2 = &st; // no prob
    // let rt3 = &mut s; // Problem. We cannot have a mutable reference while we have an immutable one to the same value.
    {
        let rt3 = &mut st; // scoping is a good hack to escape mutability restrictions
        println!("{rt3}");
    } // rt3 goes out of scope here, so we can make a new reference with no problems.
    // println!("{}, {}, and {}", rt1, rt2, rt3);

    let mut s = String::from("hello");

    //== Note that a reference’s scope starts from where it is introduced and continues through the last time that reference is used
    let r1 = &s; // no problem
    let r2 = &s; // no problem
    println!("{} and {}", r1, r2);
    // variables r1 and r2 will not be used after this point

    let r3 = &mut s; // no problem
    println!("{}", r3);
}

fn calculate_length(s: &String) -> usize {
    // s is a reference to a String
    s.len()
} // Here, s goes out of scope. But because it does not have ownership of what
  // it refers to, it is not dropped.

fn change(_some_string: &String) {
    // _some_string.push_str(", world"); // won't compile as references are imutable by defualt
}

fn change2(some_string: &mut String) {
    some_string.push_str(", world");
}

// fn dangle() -> &String { // dangle returns a reference to a String

//     let s = String::from("hello"); // s is a new String

//     &s // we return a reference to the String, s
// } // Here, s goes out of scope, and is dropped. Its memory goes away.
  // Danger!