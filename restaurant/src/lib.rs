// load contents of the module with same name
mod front_of_house;
pub use crate::front_of_house::hosting;
fn serve_order() {}

mod back_of_house {
    fn fix_incorrect_order() {
        cook_order();
        super::serve_order();
        // since relationship between back_of_house and serve_order
        // are likely to stay together -> less updates to future code
    }

    fn cook_order() {}

    /*
    Structs Public
    - need to specify public fields
    - associated methods are needed if any private
     */
    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String, // by default private
    }

    // Needs to provide a public associated function (instance Breakfast)
    // since unable to create instance with private field
    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }
    /*
    Enum Public
    - if enumal public: all variants are also public
    wouldn't be useful if private
     */
    pub enum Appetizer {
        Soup,
        Salad,
    }
}

// Symbolic link (i.e. like filesystem)
// use crate::front_of_house::hosting;
// use self::front_of_house::hosting; // same
// able to call hosting instead of crate::front_of_house::hosting

// public API
pub fn eat_at_restaurant() {
    // Absolute path
    crate::front_of_house::hosting::add_to_waitlist();
    // front_of_house is private, but fn is defined in the same mod
    // siblings can call other siblings

    // Relative path
    front_of_house::hosting::add_to_waitlist();
    // Error: Privacy boundary
    // Fix: pub: hosting, add_to_waitlist

    /* Struct Example */

    // order breakfast in the summer with Rye toast
    let mut meal = back_of_house::Breakfast::summer("Rye");
    // change mind about bread
    meal.toast = String::from("wheat");
    println!("I'd like {} toast please", meal.toast);

    // not allow to see or modify seasonal fruit Private
    // meal.seasonal_fruit = String::from("blueberries");

    /* Enum Example */
    let order1 = back_of_house::Appetizer::Soup;
    let order2 = back_of_house::Appetizer::Salad;

    /* Use Example */
    hosting::add_to_waitlist();

    /* Idiomatic use paths

    Function Scope: use parent
    instead of:
        use crate::front_of_house::hosting::add_to_waitlist;
        add_to_waitlist();

    Why?
        - Makes it clear that the function isn't locally defined

    Other Items: use full
    Why? Just convention (no good reason)
    Exception: same names are not allowed so different parent modules are needed
        - Solution: Use `as` keyword to rename into a local name/alias
    */
}

