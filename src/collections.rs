/// Vector
/*

Vec<T>
- stores more than 1 value data in a data structure
- only stores the same type
- implemented as generics
(like any other struct) will be freed when out of scope
i.e. list of items, lines of text, etc
 */

pub mod vector {
    pub fn vector_example() {
        // rare to explicitly annotate type;
        // usually type is inferred
        let v: Vec<i32> = Vec::new();
        println!("{:?}", v);

        // vec! macro for convenience
        let v = vec![1, 2, 3];
    }

    pub fn vector_update() {
        // want to change -> use mut keyword
        let mut v = Vec::new();
        v.push(5);
        v.push(6);
        v.push(7);
        v.push(8);
    } // v goes out of scope and memory freed

    /*
    Access
    1. &v[idx] -> nonexistent element attempt will crash program
    2. v.get(idx) -> returns `None` if index dne
     */
    pub fn vector_read() {
        let v = vec![1, 2, 3, 4, 5];
        let third = &v[2];
        println!("The third element is {}", third);

        match v.get(2) {
            Some(third) => println!("The third element is {}", third),
            None => println!("There is no third element"),
        }
    }

    /*
    Why should first element care about changes to the end of the vector
    - when vectors add new elements
        - requires allocating more memory
        - which might require copying old elements into a new (contiguous space)
        - thus the reference to the first element might be deallocated
     */
    pub fn enforces_ownership() {
        let mut v = vec![1, 2, 3, 4, 5]; // mutable
        let first = &v[0]; // immutable
        v.push(6);
        // println!("The first element is: {}", first);
        // Error: cannot have mutable and immutable references in the same scope
    }

    pub fn iter_values() {
        let v = vec![100, 32, 57]; // immutable
        for i in &v {
            println!("{}", i);
        }

        let mut v = vec![100, 32, 57];
        for i in &mut v {
            *i += 50;
        }
        println!("Added 50: {:?}", v);
    }

    /* Use Enums
    - types known at compile type
        - memory to heap can be allocated
    - able to be explicit in allowed types
    - enum + match => ensures that every possible case is handled
    - cons:
        - needs to know exhaustive set of types you will get at runtime
        (Traits)

    // vectors can only store the same type: inconvenient :(
    // Example: values from a row in a spreadsheet (int, floats, strings)
    // enums will hold those variants as different types
    // the vector will be the of the same type: that enum

     */
    pub fn enum_multiple_types() {
        enum SpreadsheetCell {
            Int(i32),
            Float(f64),
            Text(String),
        }

        let row = vec![
            SpreadsheetCell::Int(3),
            SpreadsheetCell::Float(10.12),
            SpreadsheetCell::Text(String::from("blue"))
        ];
    }
}

/// String
/*
core type: string slice `str`, usually used in borrowed form &str
- Both str and String are UTF-8 encoded

std lib also includes: 0sString, 0sStr, CString, CStr
 */
pub mod strings {
    pub fn create_string() {
        // Here `String::from` and `to_string` do the same thing
        let mut s = String::new();
        // new empty string `s` which can take data
        let data = "initial contents";
        let s = data.to_string();
        let s = "initial contents".to_string(); // same

        let hello = String::from("السلام عليكم");
        println!("{}", hello);
    }

    // Strings can grow in size like Vec<T>
    // `push_str` takes a string slice but doesn't take ownership
    pub fn update_string() {
        let mut s = String::from("foo");
        s.push_str("bar");
        println!("{}", s);

        // able to use s2 after s1 applies `push_str`
        // no ownership taken and code works
        let mut s1 = String::from("foo");
        let s2 = "bar";
        s1.push_str(s2);
        println!("s2 is {}", s2);

        // Add to string with a character
        s1.push('!');
        println!("s1 is {}", s1);
    }

    /*
    + Operator
    - adding a reference to the second string to the first
    - cannot add 2 String values together: can only add &str to a string

    How does &String compile (&s2) with a different parameter type?
    - compiler can coerce the &String argument into a &str
    - `add` method: Rust deref coercion: turns &s2 into &s2[..]
    - since add does not take ownership of parameter, s2 will be valid after op
    - signature:
        - takes ownership of `self`, with no &
        - so s1 will be moved into the `add` call and will not be valid after
    so:
        `let s3 = s1 + &s2;`
        does: takes ownership of s1 and appends copy of s2
        does not: create a new string and copy both strings

     */
    pub fn concat_string() {
        let s1 = String::from("Hello, ");
        let s2 = String::from("World!");
        let s3 = s1 + &s2; // s1 is moved
        // fn add(self, s: &str) -> String (not exact signature)
        // actually uses generics
    }

    // format! macro makes it easier to read
    // works like println! but returns `String` with contents
    // easier to read and no ownership taken
    pub fn concat_multiple() {
        let s1 = String::from("tic");
        let s2 = String::from("tac");
        let s3 = String::from("toe");

        // let s = s1 + "-" + &s2 + "-" + &s3;
        // println!("s is {}", s);
        // but complicated - ish :(

        let s = format!("{}-{}-{}", s1, s2, s3);
        println!("s with format! {}", s);
    }

    /*
    Indexing into Strings

    Error:
    `let s1 = String::from("hello");`
    `let h = s1[0];`

    `String` Internal Representation
    - wrapper over a Vec<u8>

    idx of string's byte will not always correlate to valid Unicode scalar value

    i.e. &"hello"[0] -> returns 104 not h
    Rust does not compile code even if it is valid because of expected values
*/
    /*
    3 Ways to look at strings:
    1. bytes
    2. scalar values
    3. grapheme clusters ("letters")

    “नमस्ते” Hindi word
    vector of u8 values
    [224, 164, 168, 224, 164, 174, 224, 164, 184, 224, 165, 141, 224, 164, 164,
224, 165, 135]
    - 18 bytes (how computer store the data)

    char type:
    ['न', 'म', 'स', '्', 'त', 'े']
    6 values, but 4th and 6th are not letters (diacritics); dont make sense alone

    grapheme clusters:
    ["न", "म", "स्", "ते"]
    - not provided in the standard library (avail. on crates.io)

    Another final reason why indexing is not allowed in Rust:
    - index should be O(1) time but not possible with rust
    - because linear search to find which characters were valid for that index
     */
    pub fn internal_rep() {
        let hello = String::from("Hola");
        let hello = String::from("Здравствуйте");
        // String length is not 12 :0
        // Rust: 25 bytes
        // Because Unicode scalar value takes 2 bytes of storage
    }

    /*
    Avoid: Slicing Strings
    - return type is not clear (byte? char? grapheme cluster? str slice?)

     */
    pub fn slice_example() {
        let hello = "Здравствуйте";
        let s = &hello[0..4];
        // each char is 2 bytes, so 4 bytes is the first 2 characters
        println!("first 2 bytes: {}", s);
    }

    // Use `chars` method: for the best way
    pub fn best_slice() {
        for c in "नमस्ते".chars() {
            println!("{}", c);
        }
    }

    pub fn slice_bytes() {
        for b in "नमस्ते".bytes() {
            println!("{}", b);
        }
    }
}

/// Hash Maps
/*
`HashMap<K, V>` mapping of type `k` keys to type `v` values
- via hashing function
(other languages call this ds: map, hash table, dict, assoc array)
- least used so not automatically included in prelude (no built-in macro)
- (like vectors) store data on heap
- All keys must be of the same type, and all values must be of the same type
 */
pub mod hash_maps {
    pub fn create() {
        use std::collections::HashMap;
        let mut scores = HashMap::new();

        scores.insert(String::from("Blue"), 10);
        scores.insert(String::from("Yellow"), 50);
    }

    /*
    collect: gathers data into a number of collection types (incl. HashMap)
    zip: creates vector of tuples ("Blue", 10)
    - collect turns vector of tuples into hashmap
     */
    pub fn itr_collect() {
        use std::collections::HashMap;
        let teams = vec![String::from("Blue"), String::from("Yellow")];
        let initial_scores = vec![10, 50];

        // Rust needs know, but able to infer based on vectors
        let mut scores: HashMap<_, _> =
            teams.into_iter().zip(initial_scores.into_iter()).collect();
    }

    /*
    Ownership
    - implementing `Copy` trait (like i32) will have values copied into hash map
    - owned values will be moved and owner will be the hashmap
     */
    pub fn ownership() {
        use std::collections::HashMap;

        let field_name = String::from("Fav color");
        let field_value = String::from("Blue");

        let mut map = HashMap::new();
        map.insert(field_name, field_value);
        // field_name and field_value are invalid at this point
        // Error
        // println!("{}", field_name);
    }

    /* Accessing values

    `get` returns Option<&V>
    - no value for that key: returns None


     */
    pub fn accessing_values() {
        use std::collections::HashMap;

        let mut scores = HashMap::new();

        scores.insert(String::from("Blue"), 10);
        scores.insert(String::from("Yellow"), 50);

        let team_name = String::from("Blue");
        let score = scores.get(&team_name);
        // returns Some(&10)

        // Arbitrary Order prints each pair
        for (key, value) in &scores {
            println!("{}: {}", key, value);
        }
    }

    /*
    Overwriting a value
     */
    pub fn overwriting_value() {
        use std::collections::HashMap;

        let mut scores = HashMap::new();

        scores.insert(String::from("Blue"), 10);
        scores.insert(String::from("Blue"), 50);

        println!("{:?}", scores);
    }
    /*
    Insert if key has no value
    - API: `entry`
        - takes key
        - returns value as enum `Entry` that rep value (might not exist)
     */
    pub fn insert_on_no_value() {
        use std::collections::HashMap;

        let mut scores = HashMap::new();
        scores.insert(String::from("Blue"), 10);

        scores.entry(String::from("Yellow")).or_insert(50);
        scores.entry(String::from("Blue")).or_insert(50);
        // checks if Yellow team has a value, if not then set to 50
        // if blue has value, then keep that value

        println!("{:?}", scores);
        // blue: 10, yellow: 50
    }

    /*
    Update based on old value
    - look up key's old value and change it
    (i.e. counting)
     */
    pub fn update_on_old_value() {
        use std::collections::HashMap;

        let text = "Hello world wonderful world";

        let mut map = HashMap::new();

        for word in text.split_whitespace() {
            // `or_insert` returns mut ref (&mut V) to the value of this key
            let count = map.entry(word).or_insert(0);
            // deference to assign value
            *count += 1;
        } // mutable ref goes out of scope

        println!("{:?}", map);
    }
}