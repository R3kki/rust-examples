/// Closures
/*
    To define code in 1 place but execute the code where result is needed
    - only 1 type (inferred)
    fn  add_one_v1   (x: u32) -> u32 { x + 1 }
    let add_one_v2 = |x: u32| -> u32 { x + 1 };
    let add_one_v3 = |x|             { x + 1 };
    let add_one_v4 = |x|               x + 1  ;

    memoization (lazy evaluation):
    - struct to hold closure and resulting value
        - each closure instance has own unique anonymous type
        - (even with the same signature, types are different)
        - `Fn` traits: `Fn`, `FnMut`, `FnOnce`
    - only executes when the value is needed and caches the result then

    Rust infers these traits based on usage
    - able to override with `move`
    FnOnce
        - takes ownership (moves into closure) of variables in enclosing scope
    FnMut
        - mutably borrows values
    Fn
        - borrows values from environment immutably
 */
use std::thread;
use std::time::Duration;

/*
    @value will be `None` before closure execution
    - when code calling `Cacher` asks for result
        -> (first time) executes, stores result in value (`Some` variant)
        -> (next time) result the result in `Some` variant
 */
struct Cacher<T>
    where
        T: Fn(u32) -> u32,
{
    calculation: T,
    value: Option<u32>,
}

// `Cacher` manages struct field values (stay private)
impl<T> Cacher<T>
    where
        T: Fn(u32) -> u32,
{
    // instance with calculation; no execution
    fn new(calculation: T) -> Cacher<T> {
        Cacher {
            calculation,
            value: None,
        }
    }
    // instead of calling closure directly, result will be held in value method
    fn value(&mut self, arg: u32) -> u32 {
        match self.value {
            Some(v) => v,
            None => {
                let v = (self.calculation)(arg);
                self.value = Some(v); // saves the result
                v // returns the value
            }
        }
    }
}

pub fn main() {
    let simulated_user_specified_value = 10; // from front-end
    let simulated_random_number = 7;

    generate_workout(simulated_user_specified_value, simulated_random_number);
}

fn generate_workout(intensity: u32, random_number: u32) {
    // |param1, param2|
    // `expensive_closure` contains the def of anonymous fn, not the return value
    let mut expensive_result = Cacher::new(|num| {
        println!("calculating slowly..");
        thread::sleep(Duration::from_secs(2));
        num
    });

    if intensity < 25 {
        println!("Today, do {} pushups!", expensive_result.value(intensity));
        println!("Next, do {} situps!", expensive_result.value(intensity));
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!("Today, run for {} minutes!", expensive_result.value(intensity));
        }
    }
}

pub fn error_example() {
    let example_closure = |x| x;

    let s = example_closure(String::from("hello"));
    // let n = example_closure(5);
}

// incurs overhead of storing values from environ
pub fn capture_environment() {
    let x = 4;
    // able to use var `x` in the same scope as fn definition
    // cannot do the same with functions
    let equal_to_x = |z| z == x;

    let y = 4;

    assert!(equal_to_x(y));
}

pub fn move_example() {
    let x = vec![1, 2, 3];
    let equal_to_x = move |z| z == x;

    // cannot use this println statement
    // println!("cannot use x here: {:?}", x);

    let y = vec![1, 2, 3];

    assert!(equal_to_x(y));
}

// Error: only works with closures since environment not captured
// pub fn fn_attempt() {
//     let x = 4;
//     fn equal_to_x(z: i32) -> bool {
//         z == x
//     }
//
//     let y = 4;
//     assert!(equal_to_x(y));
// }