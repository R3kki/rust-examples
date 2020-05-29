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

 */
pub fn panic_example() {
    panic!("crash program");
    let v = vec![1, 2, 3];
    v[99];
    panic!("test");
}
