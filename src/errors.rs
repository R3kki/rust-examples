/// Error Handling
/*
Recoverable: reasonable to report to the user and retry operation
Unrecoverable: usually a bug

- most other languages: handle both types the same
i.e. exceptions

Result<T,E> for recoverable errors
panic! for stopping execution for unrecoverable errors

`panic!`
- prints error message
- cleans stack (unwinding)
- quits
(bug)
use backtrace of `panic!` to find direct src code line
- (default is the source code of the original program)
- backtrace is a list of all functions that have been called until this point
(start from top and read until the original files)


Unwinding Stack vs Abort
- unwinding: iterates through stack and cleans up data
    - more work done by Rust
- aborting: ends program without cleaning up memory
    - OS cleans up program
    - smaller binary for release
    Cargo.toml
    [profile.release]
    panic = 'abort'

buffer overread
- in C, reading beyond end of ds is undefined
- so able to access memory at location even if memory
does belong to that ds
- leads to security vulnerabilities
- attacker can manipulate index to read prev inaccessible data

shortcuts for panic! `unwrap` and `expect`

Propagating Errors

returning error of function to the calling code (instead of handling error
within the function code)
- more control

panic! -> unrecoverable
    bad state is possible
    - assumption, guarantee, contract, invariant fails
    - not expected (not even occasionally)
    - code relies on good state
    - no good way to encode type information

Result -> calling code gets options
    - recoverable or `Err` (unrecoverable)
    - good default choice when function might fail
    - when failure is expected (malformed data, rate limit reached)

`unwrap` that panics is a placeholder
`unwrap` and `expect` : good for prototyping (not robust)

 */
pub fn panic_example() {
    panic!("crash program");
    let v = vec![1, 2, 3];
    v[99];
}

pub mod recoverable {
    pub fn open_file() {
        use std::fs::File;

        let f = File::open("hello.txt");

        let f = match f {
            Ok(file) => file,
            Err(error) => panic!("Problem opening the file: {:?}", error),
        };
    }

    pub fn handling() {
        use std::fs::File;
        use std::io::ErrorKind;

        let f = File::open("hello.txt");

        let f = match f {
            Ok(file) => file,
            Err(error) => match error.kind() {
                ErrorKind::NotFound => match File::create("hello.txt") {
                    Ok(fc) => fc,
                    Err(e) => panic!("problem creating the file: {:?}", e),
                },
                other_error => {
                    panic!("Problem opening the file {:?}", other_error)
                }
            },
        };
    }

    // same as above, no (primitive) match statements, easier to read
    pub fn handling_closures() {
        use std::fs::File;
        use std::io::ErrorKind;
        let f = File::open("hello.txt").unwrap_or_else(|error| {
            if error.kind() == ErrorKind::NotFound {
                File::create("hello.txt").unwrap_or_else(|error| {
                    panic!("problem creating the file: {:?}", error);
                })
            } else {
                panic!("Problem opening the file {:?}", error);
            }
        });
    }

    // shortcuts for panic! `unwrap` and `expect`
    pub fn unwrap_example() {
        use std::fs::File;

        let f = File::open("hello.txt").unwrap();
    }

    pub fn expect_example() {
        use std::fs::File;

        let f = File::open("hello.txt").expect("failed to open hello.txt");
    }

    pub fn propagating() {
        use std::fs::File;
        use std::io::{self, read};

        fn read_username_from_file() -> Result<String, io: Error> {
            let f = File::open("Hello.txt");

            let mut f = match f {
                Ok(file) => file,
                Err(e) => return Err(e),
            };

            let mut s = String::new();

            // returns an error that also might fail
            match f.read_to_string(&mut s) {
                Ok(_) => Ok(s),
                Err(e) => Err(e),
            }
        }

        // using the ? Operator with the same functionality
        fn read_username_from_file_operator() -> Result<String, io::Error> {
            use std::fs::File;
            use std::io::{self, read};
            let mut f = File::open("hello.txt")?;
            let mut s = String::new();
            f.read_to_string(&mut s)?;
            Ok(s);
        }

        fn shorter() {
            let mut s = String::new();

            File::open("hello.txt")?.read_to_string(&mut s)?;

            Ok(s)
        }

        fn shortest() -> Result<String, io::Error> {
            use std::fs;
            use std::io;

            // opens the file, creates new string, reads the file, put content into
            // string, and returns it
            fs::read_to_string("hello.txt")
        }
    }
}

pub mod panic_or_not {
    pub fn guess_number_old() {
        loop {
            // --snip--

            let guess: i32 = match guess.trim().parse() {
                Ok(num) => num,
                Err(_) => continue,
            };

            // tedious and impacts performance
            if guess < 1 || guess > 100 {
                println!("The secret number will be between 1 and 100.");
                continue;
            }

            match guess.cmp(&secret_number) {
                // --snip--
            }
        }
    }

    // instead: create new type and validations in a function
    // instance of type rather than validations multiple times
    // then safe for functions to use new type
    pub fn guess_number_new() {
        pub struct Guess {
            value: i32,
        }

        impl Guess {
            pub fn new(value: i32) -> Guess {
                if value < 1 || value > 100 {
                    panic!("Guess value must be between 1 and 100, got {}.", value);
                }

                Guess { value }
            }

            // getter
            pub fn value(&self) -> i32 {
                self.value
            }
        }
    }
}

