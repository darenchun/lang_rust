/*
Paths for Referring to an Item in the Module Tree.
1. absolute : starts from "crate"
2. relative : self and super
*/

pub mod front_of_house {
   pub mod hosting {
        pub fn add_to_waitlist() {
            println!("add_to_waitlist");
        }
        pub fn seat_at_table() {
            println!("seat_at_table");
        }
    }

   pub mod serving {
        pub fn take_order() {
            println!("take_order");
        }
        pub fn serve_order() {
            println!("serve_order");
        }
        pub fn take_payment() {
            println!("take_payment");
        }
    }
}

pub fn eat_at_restaurant() {
    //absolute
    crate::front_of_house::hosting::add_to_waitlist();

    //relative
    self::front_of_house::hosting::add_to_waitlist();
}
