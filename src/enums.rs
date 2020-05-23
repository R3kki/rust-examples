/// Enum
// define possible variants
// Able to put any kind of data inside enum variant:
// - strings, numeric types, structs, enum

// Any IP address can either be version 4 or 6, not both
// but either should be treated as IP addresses
enum IpAddrKind {
    V4,
    V6,
}

pub fn enum_values() {
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;

    // can take four or six
    fn route(ip_kind: IpAddrKind) {}

    // Advantage: only needs to store the type/kind, and not the data
    // right away

    struct IpAddr {
        kind: IpAddrKind,
        address: String,
    }

    let home = IpAddr {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };

    let loopback = IpAddr {
        kind: IpAddrKind::V6,
        address: String::from("::1"),
    };
}

// concise: just an enum
// rather than enum in a struct
pub fn concise_values() {
    // both V4, V6 variants have associated String values
    // share data to enum directly (no extra struct)
    enum IpAddr {
        V4(String),
        V6(String),
    }
    let home = IpAddr::V4(String::from("127.0.0.1"));
    let loopback = IpAddr::V6(String::from("::1"));
}

// Advantage: each variant can have different types and amounts of associated data
pub fn multiple_components() {
    enum IpAddr {
        // V4 will always have 4 numerical components (0 - 255)
        // not possible with a struct
        V4(u8, u8, u8, u8),
        V6(String),
    }

    let home = IpAddr::V4(127, 0, 0, 1);
    let loopback = IpAddr::V6(String::from("::1"));
}

// compare enum vs structs alone
// enums allow for conciseness, organization, and readability
pub fn compared_to_struct() {
    enum Message {
        Quit,
        // no data
        Move { x: i32, y: i32 },
        // anonymous struct
        Write(String),
        // single String
        ChangeColor(i32, i32, i32), // 3 i32 values
    }
    // equivalent
    struct QuitMessage;
    struct MoveMessage {
        x: i32,
        y: i32,
    }
    struct WriteMessage(String);
    struct ChangeColorMessage(i32, i32, i32);

    // but harder to define a function to take any of these messages

    // Define methods on enum with impl
    impl Message {
        fn call(&self) {}
    }

    let m = Message::Write(String::from("hello"));
    m.call();
}

/* Option Enum and Advantage over null values
- compiler can handle all cases and prevents bugs (which are common in other languages)
- Null feature is not in rust
- Fun Fact: Tony Hoare (inventor of null) calls it the billion-dollar mistake

Problem with null: using null as non-null value -> error
Rust encodes concept of being present/absent as Option<T>
- so useful that it's included in the prelude (already in scope)
- no need to use Option::prefix
enum Option<T> {
    Some(T),
    None,
}
 */

pub fn null_example() {
    let some_number = Some(5);
    let some_string = Some("a string");
    let absent_number: Option<i32> = None;
    // Option<T> and T(where T can be any type) are different types

    /*
     Why is this better than null?
     - able to proceed without having to check for null
     - since i8 and Option<i8> are different types: rust cannot add
     - which means the rust compiler ensures a valid value

     - possible null means explicitly using Option<T>
     - then handle the case when null, otherwise non-null
     - usually -> handle each variant (match)
     */
    let x: i8 = 5;
    let y: Option<i8> = Some(5);
    // Option<T> enum has a large number of methods to check

    // let sum = x + y;
}

/// Match
// analogy: coin-sorting machine -> coin slides into the first hole
// that fits
pub fn value_in_cents_example() {
    enum Coin {
        Penny,
        Nickel,
        Dime,
        Quarter,
    }
    fn value_in_cents(coin: Coin) -> u8 {
        // similar to if: but able to return any value (not just boolean)
        match coin {
            // instead of "if Coin == Penny"
            Coin::Penny => {
                println!("Lucky penny!");
                1
            }
            Coin::Nickel => 5,
            Coin::Dime => 10,
            Coin::Quarter => 25,
        }
    }
}

// Patterns binding to value
// match arms can bind to part of the value that match the pattern
// to extract enum variants

pub fn pattern_bind() {
    #[derive(Debug)]
    enum UsState {
        Alabama,
        Alaska,
        // etc
    }
    enum Coin {
        Penny,
        Nickel,
        Dime,
        Quarter(UsState),
    }

    // task: collecting all 50 state quarters
    fn value_in_cents(coin: Coin) -> u8 {
        match coin {
            // instead of "if Coin == Penny"
            Coin::Penny => 1,
            Coin::Nickel => 5,
            Coin::Dime => 10,
            Coin::Quarter(state) => {
                println!("State quarter from {:?}", state);
                25
            }
        }
    }
    let coin1 = value_in_cents(Coin::Quarter(UsState::Alaska));
}

pub fn option_example() {
    // if there's a value: add 1 to the value
    // no value -> then return None value and will not attempt to perform any operations
    fn plus_one(x: Option<i32>) -> Option<i32> {
        match x {
            None => None,
            Some(i) => Some(i + 1),
        }
    }
    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);
}

/* Common pattern in Rust:
- match against an enum
- bind a variable to the data
- execute code based on it
** very commonly used :_)
 */

// Matches must be exhaustive to compile
// i.e. always handles the None case by needing to explicitly check it
// in compile time

pub fn placeholder() {
    let some_u8_value = 0u8;
    match some_u8_value {
        1 => println!("one"),
        3 => println!("three"),
        5 => println!("five"),
        7 => println!("seven"),
        _ => (), // matches any value not mentioned
        // () is the unit value: nothing will happen
    }
}

// Single Case: 'if let'
// less verbose, handles one pattern and ignores the rest
pub fn if_let_example() {
    let some_u8_value = Some(0u8);
    match some_u8_value {
        Some(3) => println!("three");
        _ => (),
    }
    // want to do something special with 3 but nothing else
    // instead:
    if let Some(3) = some_u8_value {
        println!("three");
    }
}
