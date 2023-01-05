mod front_of_house {
    mod hosting {
        fn add_to_waitlist() -> ! {
            todo!()
        }

        fn seat_at_table() -> ! {
            todo!()
        }
    }

    mod serving {
        fn take_order() -> ! {
            todo!()
        }

        fn serve_order() -> ! {
            todo!()
        }

        fn take_payment() -> ! {
            todo!()
        }
    }
}

pub fn eat_at_restaurant() {
    //Absolute path
    crate::front_of_house::hosting::add_to_waitlist();

    //Relative path
    front_of_house::hosting::add_to_waitlist();
}
