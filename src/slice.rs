/// Slice
// reference a contiguous sequence of elements rather than
// the whole collection

// check for space in string and return first of string before string
pub fn first_word_independent(s: &String) -> usize {
    // since you don't want ownership '&String' works
    // but you want to return not the whole string but part of it

    // convert String to an array of bytes
    let bytes = s.as_bytes();

    // create an iterator over the array of bytes
    // iter returns each element in a collection
    // enumerate wraps the result of iter and returns each element as part of a tuple
    // (index, reference to element) -> tuple def
    for (i, &item) in bytes.iter().enumerate() {
        // byte literal syntax: search for byte that represents string
        if item == b' ' {
            return i;
        }
    }
    s.len() // otherwise return the length of the string
}

pub fn clear_example() {
    // But there's a problem: usize is independent of the string
    // no guarantee that value, or string will be valid in the future

    let mut s = String::from("hello world");
    let word = first_word(&s); // word will get value of 5
    s.clear(); // empties the string, s = ""
    // word is invalid since it holds the old value ;
    let word = first_word(&s); // word will get value of 5
    // Error prone: very tedious to sync with data every time
}

// internally: slice ds stores the start, and length of slice
// - which corresponds to the ending index
pub fn string_slices() {
    let s = String::from("string slice");
    // [starting idx..ending idx], where ending idx is not inclusive
    let s1 = &s[..6]; // similar to taking whole reference
    // same as: = &s[0..6];
    let s2 = &s[7..s.len()];
    // same as: = &s[7..12]

    println!("first word: {} \nsecond word: {}", s1, s2);
}
// Slice range index but be valid UTF-8 char boundaries
// otherwise exit in error

// &str : string slice
// compiler ensures reference to string remains valid
pub fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }
    &s[..]
}

pub fn first_word_example() {
    let mut s = String::from("hello");
    let word = first_word(&s); // needs to get a mutable reference

    // s.clear(); // adding this will cause error
    // Error: clear needs a mutable reference
    // but borrow rules disallow immutable and mutable reference

    // Compile error: word is immutable reference but it references a mutable
    println!("the first word is: {}", word);
}

pub fn slice_parameters() {
    // before: fn first_word(s: &String) -> &str {

    // allows the same function to use &String values and &str values
    // without losing functionality, makes API more general
    fn first_word(s: &str) -> &str {
        let bytes = s.as_bytes();

        for (i, &item) in bytes.iter().enumerate() {
            if item == b' ' {
                return &s[..i];
            }
        }
        &s[..]
    }

    let my_string = String::from("slice better");

    // using slice of 'String' s
    let word = first_word(&my_string[..]);

    let my_string_literal = "slice better";

    let word2 = first_word(&my_string_literal[..]);

    // since literals are also slices (already)
    let word3 = first_word(my_string_literal);
    println!("str: {}\nlit: {}\nlit no ref: {}", word, word2, word3);
}

pub fn other_slices() {
    let a = [1, 2, 3, 4, 5];
    let slice = &a[1..3];
}


