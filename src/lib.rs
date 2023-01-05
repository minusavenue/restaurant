use rand::{CryptoRng, ErrorKind::Transient, Rng};
use std::io::{self, Write};

mod front_of_house;

mod back_of_house {
    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }

        pub fn new() -> Breakfast {
            Breakfast {
                toast: String::from("Chef's Choice"),
                seasonal_fruit: String::from("Chef's Choice"),
            }
        }
    }

    pub enum Appetizer {
        Soup,
        Salad,
    }

    fn cook_order() {}

    fn fix_incorrect_order() {
        cook_order();
        super::front_of_house::serving::serve_order();
    }
}

use self::back_of_house::Appetizer;
use self::back_of_house::Breakfast;
use self::front_of_house::hosting;

pub fn eat_at_restaurant() {
    //Absolute path
    hosting::add_to_waitlist();

    let mut meal = Breakfast::summer("Rye");

    meal.toast = String::from("Wheat");

    let quick_order = Breakfast::new();

    let order1 = Appetizer::Soup;
    let order2 = Appetizer::Salad;

    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
}
