use crate::front_of_house::hosting;
pub use crate::front_of_house::hosting as front_desk;

mod front_of_house;

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
}

fn deliver_order() {}

mod back_of_house {
    fn fix_incorrect_order() {
        cook_order();
        super::deliver_order();
    }

    fn cook_order() {}

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
    }

    pub enum Appetizer {
        Soup,
        Salad,
    }
}

pub fn eat_breakfast() {
    let mut meal = back_of_house::Breakfast::summer("Rye");
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);

    // The next line won't compile if uncommented:
    // meal.seasonal_fruit = String::from("blueberries");

    let order1 = back_of_house::Appetizer::Soup;
    let order2 = back_of_house::Appetizer::Salad;
}

// Disambiguating two types named `Result` by bringing in their parent modules instead.
mod parent_module_demo {
    use std::fmt;
    use std::io;

    fn function1() -> fmt::Result {
        Ok(())
    }

    fn function2() -> io::Result<()> {
        Ok(())
    }
}

// Same disambiguation, but via the `as` keyword instead.
mod as_keyword_demo {
    use std::fmt::Result;
    use std::io::Result as IoResult;

    fn function1() -> Result {
        Ok(())
    }

    fn function2() -> IoResult<()> {
        Ok(())
    }
}

// Nested paths collapse multiple `use` lines that share a prefix.
mod nested_paths_demo {
    use std::io::{self, Write};

    fn write_something() -> io::Result<()> {
        let stdout = io::stdout();
        let mut handle = stdout.lock();
        handle.write_all(b"hello from a nested use path\n")
    }
}

// The glob operator brings every public item from a path into scope.
mod glob_demo {
    #[allow(unused_imports)]
    use std::collections::*;
}

// ── Chapter 8: Common Collections ─────────────────────────────────────────────
pub mod vectors;
pub mod strings;
pub mod hashmaps;

// ── Chapter 9: Error Handling ─────────────────────────────────────────────────
pub mod error_handling;

// 9.3 — custom validation type: panics at construction if value is out of range,
// so every live Guess is guaranteed to be 1–100.
pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 || value > 100 {
            panic!("Guess value must be between 1 and 100, got {value}.");
        }
        Guess { value }
    }

    pub fn value(&self) -> i32 {
        self.value
    }
}
