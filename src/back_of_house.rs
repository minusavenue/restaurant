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
