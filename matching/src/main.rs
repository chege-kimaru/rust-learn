#[derive(Debug)]
enum UsState {
    Alsaska,
    Alabama,
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn main() {
    println!("Value in cents {}", value_in_cents(Coin::Penny));
    println!(
        "Value in cents {}",
        value_in_cents(Coin::Quarter(UsState::Alabama))
    );

    plus_one(Some(5));
    plus_one(None);

    let dice_roll = 9;
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        // other => move_player(other), // binds to others
        // _ => reroll(), // _ is not binded
        _ => () // unit value. Do nothing
    }

    // match vs if let
    let config_max = Some(3u8);
    match config_max {
        Some(max) => println!("The maximum is configured to be {}", max),
        _ => (),
    }
    // used when the value matches one pattern and then ignores all other values.
    if let Some(max) = config_max {
        println!("The maximum is configured to be {}", max)
    }

    let mut count = 0;
    let coin = Coin::Quarter(UsState::Alabama);
    // match coin {
    //     Coin::Quarter(state) => println!("State quarter from {:?}!", state),
    //     _ => count += 1,
    // }
    if let Coin::Quarter(state) = coin {
        println!("State quarter from {:?}!", state)
    } else {
        count += 1;
    }

}

fn value_in_cents(coin: Coin) -> u8 {
    // match is exhaustive
    match coin {
        Coin::Penny => {
            println!("Lucky Penny!");
            1
        } // comma is optional here with {}
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        }
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

fn add_fancy_hat() {}
fn remove_fancy_hat() {}
fn move_player(num_spaces: u8) {}
fn reroll() {}