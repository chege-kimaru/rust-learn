use std::{fs::{File, self}, io::{ErrorKind, self, Read}};

fn main() {
    // panic!("Crush and burn");

    // let v = vec![0, 1, 2];
    // v[99];

    let greeting_file_result = File::open("hello.txt");

    let greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {:?}", error)
            },
            other_error => {
                panic!("Problem opening the file: {:?}", other_error);
            }
        },
    };

    // The above code can also be written as:
    let greeting_file = File::open("hello.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|error| {
                panic!("Problem creating the file: {:?}", error);
            })
        } else {
            panic!("Problem opening the file: {:?}", error);
        }
    });

    let greeting_file = File::open("hello.txt").unwrap(); // Will return Ok value if Result is Ok or panic for Err

    let greeting_file = File::open("hello.txt").expect("hello.txt should be included in this project"); // Just like unwrap but with an error message
}


fn read_username_from_file() -> Result<String, io::Error> {
    let username_file_result = File::open("hello.txt");

    let mut username_file = match username_file_result {
        Ok(file) => file,
        Err(e) => return Err(e)
    };

    let mut username = String::new();

    match username_file.read_to_string(&mut username) {
        Ok(_) => Ok(username),
        Err(e) => Err(e)
    }
}

// The function above can be shortedned to this using ? operator.
// The ? operator can only be used in functions whose return type is compatible with the value the ? is used on
fn read_username_from_file_2() -> Result<String, io::Error> {
    let mut username_file = File::open("hello.txt")?; // If Ok ? assigns Ok else returns Err from this function. Err is converted to the expected Err type under the hood (Using From trait)
    let mut username = String::new();
    username_file.read_to_string(&mut username)?;
    Ok(username)
}

// The function above can further be shortened by chaining
fn read_username_from_file_3() -> Result<String, io::Error> {
    let mut username = String::new();

    File::open("hello.txt")?.read_to_string(&mut username)?;

    Ok(username)
}

// Note there is actualluy a function to do this
fn read_username_from_file_4() -> Result<String, io::Error> {
    fs::read_to_string("hello.txt")
}

// "?"" can also be used with Option
fn last_char_of_first_line(text: &str) -> Option<char> {
    text.lines().next()?.chars().last()
}

// Main can also return Result. Actually it may return any types that implement the std::process::Termination trait
// fn main() -> Result<(), Box<dyn Error>> {
//     let greeting_file = File::open("hello.txt")?;

//     Ok(())
// }


// Type using structs
pub struct Guess {
    value: i32, // private
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 || value > 100 {
            panic!("Guess value must be between 1 and 100, got {}.", value);
        }

        Guess { value }
    }

    // A getter
    pub fn value(&self) -> i32 {
        self.value
    }
}