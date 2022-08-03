mod banks;
mod coin;

use coin::baht_to_usd;
use std::collections::HashMap;

// This is how you use multiple import.
use std::io::{self, Write};
use std::cmp::{Ordering, PartialEq};
// Import everything
use std::collections::*;

mod internal_module {
    fn do_something() {}
    pub mod client {
        // This is how you access parent module.
        use super::do_something;

        pub fn do_something_and_report() {
            do_something();
            println!("Report");
        }
    }
}

mod hotel {
    // Struct is also private by default.
    pub struct Breakfast {
        // you can make some field public
        pub title: String,
        price: i32,
    }

    impl Breakfast {
        // Yes, method is private by default.
        pub fn american() -> Breakfast {
            Breakfast {
                title: String::from("American breakfast"),
                price: 9999,
            }
        }
    }
}

// You can use `as` to fix conflicting name.
mod conflict_a {
    pub fn conflict() {}
}

mod conflict_b {
    pub fn conflict() {}
}

fn main() {
    println!("Hello, world!");
    println!("2 baht is {} usd", baht_to_usd(2));
    // The module is declared and imported.
    banks::deposit(999);
    banks::withdraw(999);
    internal_module::client::do_something_and_report();

    let breakfast = hotel::Breakfast::american();
    println!("I will have {}", breakfast.title);

    // `use` keyword brings a variable to current scope.
    use hotel::Breakfast;
    let another_breakfast = Breakfast::american();

    // It makes sense to use full path for a collection.
    let mut a_map: HashMap<String, i32> = HashMap::new();
    a_map.insert(String::from("key"), 32);

    use conflict_a::conflict;
    use conflict_b::conflict as conflict_from_b;
    conflict();
    conflict_from_b();

}
