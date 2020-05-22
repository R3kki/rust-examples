/// Stack and Heap
/* Runtime memory: stack and heap
stack - data must have a known and fixed size
heap - unknown size at compile time (or changes size)
OS: heap require marking an empty space big enough and returns a pointer
Allocating: use pointer to follow data stored elsewhere (heap)
*/

/* Analogy: Being seated at a restaurant
Faster: stack because the OS never has to look for space to place data
- always located at the top of the stack
Slower: heap needs to follow the pointer to get to the data
*/

/*
- Processors work better if data is closely located like stack
- function calls: values, pointers are put on the heap
- local variables inside functions: stack, values get popped off the stack
 */

pub fn variable_scope() {
    let s = "hello";
} // s is not valid out of this scope (popped off stack)

pub fn string_example() {
    // since string literals are hardcoded, immutable (convenient)
    // String type is allocated on heap since size is unknown at compile time
    // :: allows use of namespace "from" function (better than string_from name)
    let mut s = String::from("hello");
    s.push_str(", world!"); // appends a literal to a string
    println!("{}", s);
    // What is happening:
    // - String::from call implements request to memory for growing string space
    // memory is automatically returned once the variable that owns it goes out of scope
    // Rust calls special function 'drop' to return memory at end of lifetime
    // (c++) RAII - Resource Acquisition Is Initialization at the end of an item's lifetime
} // drop is called at closing } and var s is no longer valid

// 2 values of 5 are pushed onto the stack (x, y bind to those values)
pub fn move_int() {
    let x = 5;
    let y = x;
}

pub fn move_string() {
    // String is made up of 3 parts:
    // 1. pointer to memory that holds the contents of the string
    // 2. a length (current content space in bytes)
    // 3. capacity (not sure-max space? - ignored in this ex)
    // information is stored on the stack, but the actual content is held on heap
    let s1 = String::from("hello");
    let s2 = s1;
    // s2 copying s1 means:
    // s2 copies the pointer, length, capacity -> on stack
    // do not copy the underlying data

    // Double free
    // s2 and s1 both point to same location, when freeing the memory
    // they will both try to free -> security vulnerabilities
    // rust prevents invalid reference
    // println!("{}, world!", s1);
    // Error: value borrowed here after move

    // s1 variable MOVED to s1. the original s1 variable gets popped off stack
    // so rust will never automatically create deep copies of your data
}

// heap data gets copied, so both s1, s2 are pointing to separate variables
pub fn clone_string() {
    let s1 = String::from("hello");
    let s2 = s1.clone();

    println!("s1 = {}, s2 = {}", s1, s2);
}

/// Copy Trait
// older variables are still usable after assignment
// not usable if: any part implements the 'Drop' trait
// Example types
// integer (u32), bool (true, false), floats (f64), char, tuples (i32, i32)

pub fn function_example() {
    // gives_ownership() moves its return value into s1
    let s1 = gives_ownership();
    let s2 = String::from("hello");
    // s2 is moved into fn and moves return value into s3
    let s3 = takes_and_gives_back(s2);

    fn gives_ownership() -> String {
        let some_string = String::from("hello");
        some_string
    }

    fn takes_and_gives_back(a_string: String) -> String {
        a_string
    } // a_string moves out to the caller
} // s3 dropped, s2 -> already moved, s1 dropped

