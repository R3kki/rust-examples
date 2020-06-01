/// Tests
/*
test body
1. set up data, state
2. run code
3. assert results with expected

should_panic - can be imprecise, some code has panicked
    - used expected to specific the panic message expected or fail

`Result<T, E>` cannot use `#[should_panic] but returns `Err` on failure

multiple tests will run in parallel by default
- if other tests depend on previous tests then test may fail (default)
- cargo test -- --test-threads=1

show successful output of tests
cargo test -- --show-output

test function specifically
cargo test 'function name pattern match'

#[ignore]
- excludes test
-- --ignored runs only ignored tests (longer tests)

Organization
- unit tests: focused on 1 module in isolation, private interfaces
convention:
    - module: `test`
    - annotate: `cfg(test)` (compiles and runs with `cargo test`)
    - unit tests go in the same files as the code (so won't be built
        with the compiled artifact)
    - able to test private code

- integration tests: entirely external to library, only public interface
    - multiple modules per test
convention:
    - create tests dir top level (same level as src)
    - no annotation needed and compiles and runs all files in dir

avoid having file `common` in test output:
- instead of tests/common.rs
- create:tests/common/mod.rs
- this will tell Rust not to treat mod `common` as integration test

Binary crates (with src/main.rs and no src/lib.rs)
- no integration tests
- only library crates expose functions that other
crates can use since binaries run on their own

 */
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    // #[test]
    // fn fail_example() {
    //     panic!("make this test fail");
    // }

    use super::rectangle::Rectangle;

    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle {
            width: 8,
            height: 7,
        };
        let smaller = Rectangle {
            width: 5,
            height: 1,
        };

        assert!(larger.can_hold(&smaller));
    }

    use super::*;

    #[test]
    fn it_adds_two() {
        assert_eq!(4, add_two(2));
    }

    // custom failure messages
    #[test]
    fn greeting_contains_name() {
        let result = greeting("Carl");
        assert!(
            result.contains("Carl"),
            "Greeting did not contain name, instead: `{}`",
            result
        );
    }

    #[test]
    #[should_panic(expected = "Guess must be between 1 and 100, instead: 101.")]
    fn greater_than_100() {
        Guess::new(101);
    }

    // rewritten using `Result<T, E>`
    #[test]
    fn it_works2() -> Result<(), String> {
        if 2 + 2 == 4 {
            Ok(())
        } else {
            Err(String::from("2 + 2 != four"))
        }
    }
}

mod rectangle {
    #[derive(Debug)]
    pub(crate) struct Rectangle {
        pub(crate) width: u32,
        pub(crate) height: u32,
    }

    impl Rectangle {
        pub(crate) fn can_hold(&self, other: &Rectangle) -> bool {
            self.width > other.width && self.height > other.height
        }
    }
}

pub fn add_two(a: i32) -> i32 {
    a + 2
}

pub fn greeting(name: &str) -> String {
    format!("Hello {}", name)
    // bug
    // String::from("Hello!")
}

pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 || value > 100 {
            panic!("Guess must be between 1 and 100, instead: {}.", value);
        }
        Guess { value }
    }
}

