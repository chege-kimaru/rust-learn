fn main() {
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");

    // A type must be provided
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

    println!("THREE_HOURS_IN_SECONDS {THREE_HOURS_IN_SECONDS}");

    let y = 5;
    // shadowing
    let y = y + 1;

    {
        let y = y * 2;
        println!("The value of y in the inner scope is: {y}");
    }

    println!("The value of y is {y}");

    let spaces = "    ";
    let spaces = spaces.len();
    println!("spaces {spaces}");

    // You muse specify a type since there are many possible ones
    let guess: u32 = "42".parse().expect("Not a number!");
    println!("guess {guess}");

    // The suffic u8 denotes type
    let _num = 57u8;
    // _ ia a visual separator
    let num = 57_000_000;
    println!("num {num}");

    let _f = 1.0; // f64

    let _b = true; // bool is 1 byte
    let _b2: bool = false;

    let _c = 'z'; // char is 4 bytes
    let _c: char = 'z';

    // Tuples can have different types but fixed length
    let tup = (500, 6.4, 1);
    // let tup: (u32, f64, u8) = (500, 6.4, 1);
    let (x, y, z) = tup;
    println!("The values are: {x}, {y}, {z}");

    let _tup1 = tup.0;
    let tup2 = tup.1;
    println!("second element {tup2}");

    // Arrays have same type and fixed length
    // let _arr = [1, 2, 3];
    // [i32; 3] [type; length]
    let _arr: [i32; 3] = [1, 2, 3];
    let _months = ["January", "February", "March", "April", "May", "June", "July",
              "August", "September", "October", "November", "December"];
    // setting initial value
    let arr2 = [3; 5]; // = let a = [3, 3, 3, 3, 3];
    println!("arr2, {}", arr2[0])
}
