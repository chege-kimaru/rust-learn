fn main() {
    println!("Hello, world!");

    another_function(5);

    let y = {
        let x = 3;
        x + 1 // This is an expression. so no semi-colon at the end which would make it a statement
    };
    println!("y = {y}");

    let x  = five();
    println!("x is {x}");
}

fn another_function(x: i32) {
    println!("The value of x is: {x}");
}

fn five() -> i32 {
    5 // return value.
}