use std::collections::HashMap;

fn main() {
    // ======= Vectors

    let v: Vec<i32> = Vec::new();

    let v = vec![1, 2, 3];

    let mut v = Vec::new();
    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);

    let third: &i32 = &v[2];
    println!("The third element is {third}");

    let third: Option<&i32> = v.get(2);
    match third {
        Some(third) => println!("The third element is {third}"),
        None => println!("There is no third element")
    }

    // let first = &v[0]; // borrowed as immutable
    // v.push(9); // cannot be borrowed as mutable. Error
    // println!("The first element is {first}");

    for i in &v {
        println!("{i}");
    }

    let mut v = vec![100, 32, 57];
    for i in &mut v {
        *i += 50; // * dereferences &i
    }

    enum  SpreadSheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadSheetCell::Int(3),
        SpreadSheetCell::Text(String::from("blue")),
        SpreadSheetCell::Float(10.12),
    ];


    //======== Strings

    let mut s = String::new();

    let s = "Initial contents".to_string(); // create String from string literal
    let s = String::from("Initial contents");

    let mut s = String::from("foo");
    s.push_str("bar");

    let mut s = String::from("lo");
    s.push('l');

    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2; // note s1 has been moved here and can no longer be used

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");
    let s = s1 + "-" + &s2 + "-" + &s3;

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");
    let s = format!("{s1}-{s2}-{s3}"); // takes no ownership

    let s1 = String::from("hello");
    // let h = s1[0]; // Error
    let s = &s1[0..2];

    for c in "hello".chars() {
        println!("{c}");
    }
    for b in "hello".bytes() {
        println!("{b}");
    }

    // ===== hashmaps

    // creating
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    // Get value
    let team_name = String::from("Blue");
    // get returns an option. This program handles the Option by calling copied to get an Option<i32> rather than an Option<&i32>, then unwrap_or to set score to zero if scores doesn't have an entry for the key
    let score = scores.get(&team_name).copied().unwrap_or(0);

    // loop
    for (key, value) in &scores {
        println!("{key}: {value}");
    }

    // ownership
    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();
    map.insert(field_name, field_value);
    // field_name and field_value are invalid at this point, try using them and
    // see what compiler error you get!
    // If we insert references to values into the hash map, the values won’t be moved into the hash map. 
    // The values that the references point to must be valid for at least as long as the hash map is valid.

    // updating
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Blue"), 25);

    println!("{:?}", scores);

    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);

    scores.entry(String::from("Yellow")).or_insert(50);
    scores.entry(String::from("Blue")).or_insert(50);

    println!("{:?}", scores);

    let text = "hello world wonderful world";
    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }
    println!("{:?}", map);
}
