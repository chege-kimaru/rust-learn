fn main() {
    // loop {
    //     println!("Hello, world!");
    // }

    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {result}");


    let mut count = 0;
    // loop label
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                // exists the inner loop
                break;
            }
            if count == 2 {
                // breaks out of the labeled loop
                break 'counting_up;
            }
            remaining -= 1;
        }
        count += 1;
    }
    println!("End count = {count}");

    let mut number = 3;

    while number != 0 {
        println!("{number}");
        number -= 1;
    }
    println!("LIFTOFF!!!");

    let a = [10, 20, 30, 40 ,50];
    let mut index = 0;
    while index < 5 {
        println!("Value is {}", a[index]);
        index += 1;
    }

    for el in a {
        println!("The value is {el}");
    }

    // rev reverses a range
    for number in (1..=4).rev() {
        println!("{number}!");
    }
    println!("LIFTOFF!!!");
}
