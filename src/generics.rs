/// Generics
/*
abstract types to express behaviour without knowing
what exact type will be in their place

Enums or structs with only a type difference:
- can be replaced with generic types

Performance is the same as concrete types!
- monomorphization process is used at compile time (generic into specific types)
(compiler does the opposite work of the generic process)

Conditionals
blanket implementations: implementations of a trait that satisfy the bounds of another trait

i.e. std library implements `ToString` trait on any type that implements `Display` trait
impl <T: Display> ToString for T {}

let s = 3.to_string() works because Integers implement `Display`

Traits and Trait bound: able to use generic type parameters
 -> will not compile// here result has a lifetime creater than its sler checks all the concrete types at compile time
- so not type checking tests are needed

Lifetime Annotations
- names must start with ', are usually all lowercase and very short
Example
&i32        // reference
&'a i32     // reference with explicit lifetime
&'a mut i32 // mutable reference to i32 with lifetime of 'a

- lifetime annotation go in the signature not the body
- return references with smallest lifetime parameter

- lifetime syntax is about connecting lifetimes of parameters and return types so compiler runs memory-safe op
(i.e. no dangling pointer)

- every reference has a lifetime

Lifetime Elision Rules
- rust programmers found patterns that are deterministic and cleaner (less explicit lifetimes ref)
- set of particular cases that compiler will consider

input lifetimes: on functions, method parameters
output lifetimes: return values

3 Rules: (if any references unaccounted for, then lifetimes that are not explicit will result in an error)
1. each reference parameter gets its own lifetime
    fn foo<'a>(x: &'a i32) -> function with 1 parameter = 1 lifetime parameter
    fn foo<'a, 'b>(x: &'a i32, y: &'b i32) -> function with 2 parameter = 2 lifetime parameter
2. if 1 input lifetime parameter exists, all output lifetime parameters will be assigned the same
    fn foo<'a>(x: &'a i32) -> &'a i32
3. for methods: if 1 input lifetime parameter is `&self` or `&mut self` -> (b/c method) lifetime of `self` is assigned to all output
lifetimes (fewer symbols needed)

Example 1: fn first_word(s: &str) -> &str {
compiler applies the rules:
#1: fn first_word<'a>(s: &'a str) -> &str {
#2: fn first_word<'a>(s: &'a str) -> &'a str {

-> now all the references have lifetimes and compiles

Example 2: fn longest(x: &str, y: &str) -> &str {
#1: fn longest<'a, 'b>(x: &'a str, y: &'a str) -> &str {
#2: fn longest<'a, 'b>(x: &'a str, y: &'a str) -> &str {
#3: doesn't apply since not a method and no &self
-> but return type lifetime missing -> Error

Static Lifetime
- exists until the end of the program
- is directly stored in the binary
 */

pub mod remove_duplication {
    pub fn extract_function_example() {
        fn largest(list: &[i32]) -> i32 {
            let mut largest = list[0];

            for &item in list {
                if item > largest {
                    largest = item;
                }
            }
            largest
        }
        let number_list = vec![34, 50, 25, 100, 65];
        println!("The largest number is {}", largest(&number_list));

        let number_list = vec![102, 34, 6000, 89, 54, 2, 43, 8];
        println!("The largest number is {}", largest(&number_list));
    }

    pub fn generic_data_types() {
        fn largest_i32(list: &[i32]) -> i32 {
            let mut largest = list[0];

            for &item in list {
                if item > largest {
                    largest = item;
                }
            }
            largest
        }

        fn largest_char(list: &[char]) -> char {
            let mut largest = list[0];

            for &item in list {
                if item > largest {
                    largest = item;
                }
            }
            largest
        }

        // Old
        // fn largest<T>(list: &[T]) -> T {

        // With Traits
        // Adding + Copy solves:
        // specifies the generic parameter to have traits that implement the  `Copy` trait
        // because Partial0rd by itself would result in error if some values could be move and others not
        fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
            let mut largest = list[0];

            for &item in list {
                if item > largest {
                    largest = item;
                }
            }
            largest
        }

        let number_list = vec![34, 50, 25, 100, 65];

        let result = largest_i32(&number_list);
        println!("The largest number is {}", result);

        let char_list = vec!['y', 'm', 'a', 'q'];

        let result = largest_char(&char_list);
        println!("The largest char is {}", result);
    }
}


pub mod struct_def {
    pub fn example() {
        // Value of T is the same for both x, and y
        struct Point<T> {
            x: T,
            y: T,
        }
        // Tradeoff: harder to read, but good for simple code
        // i.e. Result<T, E> {
        // Ok(T), Err(E), }
        struct Point2<T, U> {
            x: T,
            y: U,
        }

        let point1 = Point { x: 1, y: 2 };
        println!("Point1 is: {}, {}", point1.x, point1.y);
        let point1 = Point { x: 1.5, y: 2.6 };
        println!("Point1 is: {}, {}", point1.x, point1.y);

        let point2 = Point2 { x: 1, y: 2.6 };
        println!("Point2 is: {}, {}", point2.x, point2.y);
    }
}
/*
Method

- named x on Point<T>
- impl<T> tells compiler that generic type is used
- or can specify point
 */
pub mod method_def {
    pub fn example() {
        struct Point<T> {
            x: T,
            y: T,
        }

        // applies to all other types that are not f32
        impl<T> Point<T> {
            fn x(&self) -> &T {
                &self.x
            }
        }
        // only applies to f32
        impl Point<f32> {
            fn distance_from_origin(&self) -> f32 {
                (self.x.powi(2) + self.y.powi(2)).sqrt()
            }
        }

        let p = Point { x: 1, y: 2 };

        println!("p.x = {}", p.x());
    }

    pub fn mixup() {
        struct Point<T, U> {
            x: T,
            y: U,
        }

        impl<T, U> Point<T, U> {
            fn mixup<V, W>(self, other: Point<V, W>) -> Point<T, W> {
                Point {
                    x: self.x,
                    y: other.y,
                }
            }
        }
        let p1 = Point { x: 1, y: 2.2 };
        let p2 = Point { x: "Hi", y: "Earthling" };

        let p3 = p1.mixup(p2);
        println!("p3 is x={}, y={}", p3.x, p3.y);
    }
}

pub mod performance {
    // Monomorphization process through an example
    pub fn example() {
        // Option<T>
        let integer = Some(5);
        let float = Some(5.0);

        // What happens to the code at compile time (Monomorphization process)
        enum Option_i32 {
            Some(i32),
            None,
        }
        enum Option_f64 {
            Some(f64),
            None,
        }
        // in the src/main.rs
        let integer = Option_i32::Some(5);
        let float = Option_f64::Some(5.0);
    }
}

/*
Traits

- shared behaviour in an abstract way (behaviour as in methods)
- similar to interfaces in other languages (with differences)

What is a Trait?
- different types can have the same behaviour (able to call the same method() )
- i.e group method signatures
- Each type implementing a trait must provide implementation body

coherence, orphan rule prevent external traits from implementing already defined types
- so external code cannot break crate and vice versa
- and rust would not know how to implement the same type for the same trait
 */
pub mod traits {
    pub trait Summary {
        // Method signature only: no body implementation
        fn summarize(&self) -> String {
            String::from("(Read more...)")
        }
    }

    pub struct NewsArticle {
        pub headline: String,
        pub location: String,
        pub author: String,
        pub content: String,
    }

    //
    /*

    impl Summary for NewsArticle {
        fn summarize(&self) -> String {
            format! {"{} by {} ({})", self.headline, self.author, self.location}
        }
     */

    impl Summary for NewsArticle {}

    pub struct Tweet {
        pub username: String,
        pub content: String,
        pub reply: bool,
        pub retweet: bool,
    }

    impl Summary for Tweet {
        fn summarize(&self) -> String {
            format!("{}: {}", self.username, self.content)
        }
    }

    pub fn tweet_example() {
        let tweet = Tweet {
            username: String::from("pwn_ebooks"),
            content: String::from("This contest is the best"),
            reply: false,
            retweet: false,
        };

        println!("1 new tweet: {}", tweet.summarize());
    }

    pub fn article_default() {
        let article = NewsArticle {
            headline: String::from("Title"),
            location: String::from("location"),
            author: String::from("author"),
            content: String::from("This is the content body"),
        };

        println!("News article is {}", article.summarize());
    }

    pub mod default {
        pub trait Summary {
            fn summarize_author(&self) -> String;

            fn summarize(&self) -> String {
                format!("(Read more from {}...)", self.summarize_author())
            }
        }

        pub struct Tweet {
            pub username: String,
            pub content: String,
            pub reply: bool,
            pub retweet: bool,
        }

        impl Summary for Tweet {
            fn summarize_author(&self) -> String {
                format!("@{}", self.username)
            }
        }

        pub fn example() {
            let tweet = Tweet {
                username: String::from("horse_ebooks"),
                content: String::from(
                    "of course, as you probably already know, people",
                ),
                reply: false,
                retweet: false,
            };

            println!("1 new tweet: {}", tweet.summarize());
        }
    }

    pub fn parameter() {
        // cleaner syntax
        pub fn notify(item: impl Summary) {
            println!("Breaking news! {}", item.summarize());
        }
        // Trait bound
        pub fn notify2<T: Summary>(item: T) {
            println!("Breaking news! {}", item.summarize());
        }
        // impl syntax is better for different types
        pub fn notify3(item1: impl Summary, item2: impl Summary) {}
        // Trait bound is better for same types
        pub fn notify4<T: Summary>(item1: T, item2: T) {}
    }

    pub fn multiple_trait_bounds() {
        // Specify more than 1 trait bound
        // pub fn notify(item: imply Summary + Display) {}
        // pub fn notify2<T: Summary + Display>(item: T) {}
    }

    pub fn where_clause() {
        /* make multiple trait bounds easier to read

        fn some_function<T: Display + Clone, U: Clone + Debug>(t: T, u: U){}
        fn some_function<T, U>(t: T, u:U){
            where T: Display + Clone,
                  U: Clone + Debug
        }

         */
    }

    // Returns some type of Summary without specifying the contrete type of Tweet
    pub fn return_trait() -> impl Summary {
        Tweet {
            username: String::from("horse_ebooks"),
            content: String::from(
                "of course, as you probably already know, people",
            ),
            reply: false,
            retweet: false,
        }
    }
    // Only allowed to return 1 type (no switch cases)


    pub mod conditionals {
        use std::fmt::Display;

        struct Pair<T> {
            x: T,
            y: T,
        }

        impl<T> Pair<T> {
            fn new(x: T, y: T) -> Self {
                Self { x, y }
            }
        }

        impl<T: Display + PartialOrd> Pair<T> {
            fn cmp_display(&self) {
                if self.x >= self.y {
                    println!("The largest member is x = {}", self.x);
                } else {
                    println!("The largest member is y = {}", self.y);
                }
            }
        }
    }
}

/*
One of Rust most distinct features
 */
pub mod lifetimes {
    pub fn prevent_dangling_ptr() {
        // compile time error (no null value)
        // let r;
        // {
        //     let x = 5;
        //     r = &x;
        // }
        // println!("r: {}", r);

        // fixes compile time error with scope
        let x = 5;
        let r = &x;
        println!("r is {}", r);
    }

    pub fn generic_functions() {
        // use self::super::remove_duplication::generic_data_types;

        // Needs a generic lifetime parameter because return type doesn't know if return will be x or y
        // all ref in the parameters and return type must have the same lifetime

        /*
        for a lifetime of 'a, two string ref (parameters) will live as long as 'a
         */
        fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
            if x.len() > y.len() {
                x
            } else {
                y
            }
        }

        // string1 valid until end of block, string2 valid only within the inner block
        // so result references until the end of the inner scope
        let string1 = String::from("long string is long");

        {
            let string2 = String::from("xyz");
            let result = longest(string1.as_str(), string2.as_str());
            println!("The longest string is {}", result);
        }
        /*
        Example that won't compile
        let string1 = String::from("long string is long");
        let result;
        {
            let string2 = String::from("xyz");
            let result = longest(string1.as_str(), string2.as_str());
        }
        // here result has a lifetime creater than its arguments -> will not compile
        println!("The longest string is {}", result);


         */
    }

    pub fn struct_def() {
        // Instance of ImportantExcerpt cannot outlive the reference part it holds
        struct ImportantExcerpt<'a> {
            part: &'a str,
        }
        // first_sentence data is owned by novel, and novel doesn't go out of scope before ImportantExcerpt OK
        let novel = String::from("Once upon a time. There was a boy.");
        let first_sentence = novel.split('.').next().expect("Could not find a '.'");
        let i = ImportantExcerpt {
            part: first_sentence,
        };
    }

    /*
    pre 1.0 versions, all references would need an explicit lifetime

     */
    pub fn elision() {
        // Code compiles without lifetimes
        // i.e. fn first_word<'a>(s: &'a str) -> &'a str {
        fn first_word(s: &str) -> &str {
            let bytes = s.as_bytes();

            for (i, &item) in bytes.iter().enumerate() {
                if item == b' ' {
                    return &s[0..i];
                }
            }

            &s[..]
        }

        // Method on a struct with lifetimes, uses same syntax as generics -> often lifetimes annotation not needed
        fn method() {
            struct ImportantExcerpt<'a> {
                part: &'a str,
            }

            impl<'a> ImportantExcerpt<'a> {
                // method level, with only parameter &self and no reference return type
                fn level(&self) -> i32 {
                    3
                }
            }
            // no explicit lifetimes needed
            impl<'a> ImportantExcerpt<'a> {
                fn announce_and_return_part(&self, announcement: &str) -> &str {
                    println!("Attention please: {}", announcement);
                    self.part
                }
            }
        }

        // Special case
        fn static_lifetime() {
            let s: &'static str = "This is a static lifetime";
        }
    }
}

// Combining generic type parameters, trait bounds, and lifetimes in one function
pub mod together {
    /*
    Longest function
    - returns the longer of two strings
    - ann (generic type: T) which can be any type that implements the Display trait (prints out)

     */
    fn example() {
        use std::fmt::Display;

        // T is a trait bound, 'a is the lifetime
        fn longest_with_an_announcement<'a, T>(
            x: &'a str,
            y: &'a str,
            ann: T,
        ) -> &'a str
            where
                T: Display,
        {
            println!("Announcement {}", ann);
            if x.len() > y.len() {
                x
            } else {
                y
            }
        }
    }
}
