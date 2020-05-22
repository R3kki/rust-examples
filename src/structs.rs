/// Structs
// different from tuples: different types involved
// needs naming of values
// will not rely on order (like tuples) - instead on the named value
/// Summary
// - custom types to give meaning to domain
// - keep associated pieces of data connected
// - clear naming
// methods: allow behaviour of structs
// associated functions: namespace functionality without instance

pub fn basic_struct() {
    // keyword: User - names the entire struct
    struct User {
        // fields: define name and types of data
        username: String,
        email: String,
        sign_in_count: u64,
        active: bool,
    }

    // to use after defining struct: create instance
    // {} contains key: value pairs, no order
    let mut user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("username123"),
        active: true,
        sign_in_count: 1,
        // returns User as an expression
    };

    // get a specific value with the dot notation
    user1.email = String::from("anotheremail@example.com");
    // the whole instance must be mutable to change field
    // cannot just make fields mutable

    // using shorthand (when variable and field names are the same)
    fn build_user(email: String, username: String) -> User {
        User {
            email,
            username,
            active: true,
            sign_in_count: 1,
        }
    }

    // Struct Update syntax (resume old instance values, change others)
    let user2 = User {
        email: String::from("another@example.com"),
        username: String::from("anotherusername567"),
        ..user1
        // .. means sets the remaining fields as the given instance
        // active: user1.active,
        // sign_in_count: user1.sign_in_count,
    };
}

pub fn tuple_struct_simple() {
    // struct have looks similar to tuples (fields don't have names)
    struct Color(i32, i32, i32);
    struct Point(i32, i32, i32);

    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
    // black and origin are different types -> instance of diff tuple structs
    // so even tho the arguments are the same -> type Color != type Point

    // instances behave like tuple: destructure with '.' with index
    let black_part = black.0;
}

pub fn ownership_struct() {
    // String type means instance of struct owns all the data
    // - all the data wil be valid as long as the entire struct is valid

    struct User {
        // username: &str, // needs lifetime specifiers
        // email: &str, // needs lifetime specifiers
        sign_in_count: u64,
        active: bool,
    }

    // let user1 = User {
    //     email: "someone@example.com",
    //     username: "username123",
    //     active: true,
    //     sign_in_count: 1,
    // };
}

pub fn rectangles() {
    let width1 = 30;
    let height1 = 50;

    println!(
        "The area of the rectangle is {} square pixels",
        area1(width1, height1)
    );

    // Issue: the area is suppose to calculate the area of 1 rectangle
    // but the function has 2 parameters (independent)
    fn area1(width: u32, height: u32) -> u32 {
        width * height
    }

    //
    // Refactor with tuples
    //
    let rect1 = (30, 50);

    println!(
        "The area of the rectangle is {} square pixels",
        area2(rect1)
    );

    fn area2(dimensions: (u32, u32)) -> u32 {
        dimensions.0 * dimensions.1
    }
    // tuples adds structure, only passing in 1 argument
    // not clear: elements not named, and indexs are confusing

    /* Refactor structs with more meaning */

    struct Rectangle {
        width: u32,
        height: u32,
    }

    let rect2 = Rectangle {
        width: 30,
        height: 50,
    };

    println!(
        "The area of the rectangle is {} square pixels",
        area3(&rect2)
    );

    // immutable borrow -> so that the main retains ownership and continue
    // using it
    fn area3(rectangle: &Rectangle) -> u32 {
        rectangle.width * rectangle.height
    }
}

pub fn good_struct() {
    // needs to explicitly opt in functionality for struct
    #[derive(Debug)]
    struct Rectangle {
        width: u32,
        height: u32,
    }

    let rect = Rectangle {
        width: 30,
        height: 50,
    };

    fn area(rectangle: &Rectangle) -> u32 {
        rectangle.width * rectangle.height
    }

    // :? uses output format called Debug
    // Debug is a trait that enables printing struct (used by devs)
    println!("rect is {:?}", rect);

    // :#? easier to read
    println!("rect is {:#?}", rect);
}

/// Method Syntax
// similar to functions
// - declared with fn, have parameters, return value
// defined in the context of a struct (or enum or trait)
// first parameter is always self (representing the instance of the struct)
/*
Main benefit:
- organization
- put all things we can do with an instance of a type in one impl block
- rather than having to search for capabilities in various places of lib
 */
pub fn method_struct() {
    #[derive(Debug)]
    struct Rectangle {
        width: u32,
        height: u32,
    }

    impl Rectangle {
        // define function within context of Rectangle
        // change parameter to self
        // instead of rectangle: &Rectangle
        // Rust knows type of self is Rectangle (inside Rectangle context)
        fn area(&self) -> u32 { // borrow self immutably
            // (Rare) &mut self is able to change the instance
            // method transforms self into something else
            // and want ot prevent the caller from using the og instance
            // after the transformation
            self.width * self.height
        }
    }

    let rect1 = Rectangle {
        width: 3,
        height: 5,
    };

    println!(
        "The area of the rect is {} square pixels",
        rect1.area()
    );
}

/* Automatic referencing and deref: -> Operator c/c++ Equivalent in Rust
'.' calling methods
'->' calling a method on the object directly
 and calling method on a ptr to obj and dereference ptr
 i.e. obj is ptr, obj->smthg() is similar to (*obj).smthg()

 Rust instead does this automatically
 i.e. calling methods
 rust wil automatically add: "&", "&mut" or "*" to match signature
 p1.distance(&p2) is same as (&p1).distance(&p2);
 // first is cleaner: possible b/c of clear receiver of type: self
 - borrowing is implicit for method receivers -> makes it easy to find out
 */

pub fn more_params() {
    #[derive(Debug)]
    struct Rectangle {
        width: u32,
        height: u32,
    }
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));

    // Task: define a method for Rectangle, immutable borrow of another Rect
    // just need to read the value (so mutable borrow not needed)
    // so main retains ownership of rect2 so main can use rect2 after func call

    impl Rectangle {
        fn area(&self) -> u32 {
            self.width * self.height
        }

        fn can_hold(&self, other: &Rectangle) -> bool {
            self.width > other.width && self.height > other.height
        }
    }
}

/* Associated Functions
- "associated" with struct
- define functions within the impl block
- do not take self as parameter
- function and not a method because they dont't have an instance
of the struct to work with
i.e. String::from is an associated function

use with '::'
function is namespace by the struct
("::" syntax is used for both associated functions and namesapces
created by modules)

 */
pub fn associated_functions() {
    #[derive(Debug)]
    struct Rectangle {
        width: u32,
        height: u32,
    }

    impl Rectangle {
        fn square(size: u32) -> Rectangle {
            Rectangle {
                width: size,
                height: size,
            }
        }
    }

    // call associated function with '::'
    let square1 = Rectangle::square(3);

    println!("square1 = {:#?}", square1);
}

// able to have multiple impl blocks
// i.e. separate the functions in each own impl but no reason to do so
// use case: generic types and traits
