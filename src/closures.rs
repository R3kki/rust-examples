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

pub mod iterators {
    pub fn example() {
        let v1 = vec![1, 2, 3];

        let v1_iter = v1.iter();

        for val in v1_iter {
            println!("got: {}", val);
        }
    }

    // Filter iterator adapter
    #[derive(PartialEq, Debug)]
    struct Shoe {
        size: u32,
        style: String,
    }

    fn shoes_in_my_size(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {
        shoes.into_iter().filter(|s| s.size == shoe_size).collect()
    }


    #[cfg(test)]
    pub mod tests {
        use self::super::*;

        #[test]
        fn iter_demo() {
            let v1 = vec![1, 2, 3];
            let mut v1_iter = v1.iter();

            assert_eq!(v1_iter.next(), Some(&1));
            assert_eq!(v1_iter.next(), Some(&2));
            assert_eq!(v1_iter.next(), Some(&3));
            assert_eq!(v1_iter.next(), None);
        }

        #[test]
        fn iter_sum() {
            let v1 = vec![1, 2, 3];

            let v1_iter = v1.iter();

            let total: i32 = v1_iter.sum();

            assert_eq!(total, 6);
        }

        #[test]
        pub fn map_adapter() {
            let v1: Vec<i32> = vec![1, 2, 3];

            let v2: Vec<_> = v1.iter().map(|x| x + 1).collect();

            assert_eq!(v2, vec![2, 3, 4]);
        }

        #[test]
        fn filters_by_size() {
            let shoes = vec![
                Shoe {
                    size: 10,
                    style: String::from("sneaker"),
                },
                Shoe {
                    size: 13,
                    style: String::from("sandal"),
                },
                Shoe {
                    size: 10,
                    style: String::from("boot"),
                },
            ];

            let in_my_size = shoes_in_my_size(shoes, 10);

            assert_eq!(
                in_my_size,
                vec![
                    Shoe {
                        size: 10,
                        style: String::from("sneaker"),
                    },
                    Shoe {
                        size: 10,
                        style: String::from("boot"),
                    },
                ]
            );
        }

        #[test]
        fn calling_next_directly() {
            let mut counter = Counter::new();

            assert_eq!(counter.next(), Some(1));
            assert_eq!(counter.next(), Some(2));
            assert_eq!(counter.next(), Some(3));
            assert_eq!(counter.next(), Some(4));
            assert_eq!(counter.next(), Some(5));
            assert_eq!(counter.next(), None);
        }

        // take instance Counter
        // pair with another counter instance (after skipping first instance)
        // multiply each pair
        // keep only those divisible by 3
        // and return sum of returning values
        #[test]
        fn using_other_itr_trait_methods() {
            let sum: u32 = Counter::new()
                .zip(Counter::new().skip(1)) // (5, None) pair is never produced
                .map(|(a, b)| a * b)
                .filter(|x| x % 3 == 0)
                .sum();
            assert_eq!(18, sum);
        }
    }

    struct Counter {
        count: u32
    }

    impl Counter {
        fn new() -> Counter {
            Counter { count: 0 }
        }
    }

    impl Iterator for Counter {
        type Item = u32;

        fn next(&mut self) -> Option<Self::Item> {
            if self.count < 5 {
                self.count += 1;
                Some(self.count)
            } else {
                None
            }
        }
    }
}

pub mod performance {
    /*
    Itr are an example of zero-cost abstractions (and still remains high-level)
    linear prediction math to estimate future values based on linear fn of prev samples
    itr chain on 3 variables: `buffer` slice of data, `coefficients` array, `qlp_shift` shift
     */
    pub fn audio_decoder() {
        /*

        let buffer: &mut [i32];
        let coefficients: [i64; 12];
        let qlp_shift: i16;

        for i in 12..buffer.len() {
            let prediction = coefficients.iter()
                .zip(&buffer[i - 12..i])
                .map(|(&c, &s)| c * s as i64)
                .sum::<i64>() >> qlp_shift;
            let delta = buffer[i];
            buffer[i] = prediction as i32 + delta;
        }
         */
    }
}