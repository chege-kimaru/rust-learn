mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}

        fn seat_at_table() {}
    }

    mod serving {
        fn take_order() {}

        fn serve_order() {}

        fn take_payment() {}
    }
}

fn deliver_order() {}

mod back_of_house {
    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast { toast: String::from(toast), seasonal_fruit: String::from("peaches") }
        }
    }

    pub enum Appetizer {
        Soup,
        Salad,
    }


    fn fix_incorrect_order() {
        cook_order();
        // refrence item in the parent module
        super::deliver_order();
    }

    fn cook_order() {}
}

use crate::front_of_house::hosting;
// pub use crate::front_of_house::hosting; // To expose hosting to outside code. To be used as restaurant::hosting::add_to_waitlist()

pub fn eat_at_restaurant() {
    // absolute path
    crate::front_of_house::hosting::add_to_waitlist();

    hosting::add_to_waitlist(); // after adding use keyword

    // relative path
    front_of_house::hosting::add_to_waitlist();

    let mut meal = back_of_house::Breakfast::summer("Rye");
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);

    let order1 = back_of_house::Appetizer::Soup;
    let order2 = back_of_house::Appetizer::Salad;
}

mod customer {

    use super::hosting;

    pub fn eat_at_restaurant() {
        hosting::add_to_waitlist();
    }
}

// Method 1 of shared struct name
use std::fmt; // use fmt::Result
use std::io; // use io::Result
// Method 2. Using as keyword
use std::fmt::Result;
use std::io:: Result as IoResult;


// Using Nested paths
// use std::cmp::Ordering;
// use std::collections;
use std::{cmp::Ordering, collections};
// use std::io;
// use std::io::Write;
use std::{self, write};

// Glob operator
use std::collections::*;
