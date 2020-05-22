/// References and Borrowing
/// Rules:
/// 1. Either 1 mut ref or any number of immutable references
/// 2. References always be valid, and go out of scope before the data

pub fn basic_reference() {
    // takes reference instead of taking ownership
    let s1 = String::from("hello");
    let len = calculate_length(&s1);
    println!("The length of '{}' is {}", s1, len);

    // &String refers to value without taking ownership
    fn calculate_length(s: &String) -> usize {
        // since s is referenced and thus not owned
        // after caller goes out of scope, it will not drop value
        s.len()
    } // s out of scope -> nothing happens to s
}

/// Borrowing:
/// function parameters are references instead of actual values
/// never had ownership so won't need to return the values
pub fn basic_borrow() {
    // References are immutable by default so must use mut
    let mut s = String::from("basic borrow");
    change(&mut s);

    fn change(some_string: &mut String) {
        some_string.push_str(", fn");
    }
    println!("s is: {}", s);
}

// Mutable reference restriction: only 1 mut reference per data per scope
pub fn mutable_wrong() {
    let mut s = String::from("hello");
    let r1 = &mut s;
    let r2 = &mut s;
    // println!("{}, {}", r1, r2);
    // Compiler Error to use more than 1 mutable reference at a time
}

/// Prevents dta race
/// - 2 or more pointers accessing data at the same time
/// and at least of them is writing to the data
/// - no way to sync the access to the data

pub fn mutable_scope() {
    let mut s = String::from("hello");
    {
        let r1 = &mut s;
    } // r1 goes out of scope, able to make new references
    let r2 = &mut s;
    println!("{}", r2);
}

pub fn mutable_immutable_wrong() {
    let mut s = String::from("hello");
    let r1 = &s; // no problem
    let r2 = &s; // no problem
    let r3 = &mut s; // no problem
    // println!("{}, {}, and {}", r1, r2, r3);
    // Compiler Error: cannot use another reference
    // if another one is declared mutable
}

pub fn mutable_multiple() {
// Reference scope ends the last time is is used.
    let mut s = String::from("hello");
    let r1 = &s; // no problem
    let r2 = &s; // no problem

    println!("{} and {}", r1, r2);
    // r1 and r2 are no longer used -> so scope ends here

    let r3 = &mut s; // no problem
    println!("{}", r3);
}

pub fn dangling_ptr() {
    // pointer that references memory, but already freed by someone else
    // Rust guarantees no dangling references
    // compiler ensures references will go out of scope before data

    let reference_to_nothing = dangle();

    /*
    fn dangle() -> &String { // returns reference to string
        let s = String::from("hello");
        &s // return a reference to a string
    } // here s goes out of scope -> dropped, but the reference to s is returned
    // Compiler Error
     */
    fn dangle() -> String {
        let s = String::from("hello");
        s;
    }
    // Ownership is moved out, nothing gets deallocated
}
