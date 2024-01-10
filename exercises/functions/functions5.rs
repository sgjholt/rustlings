// functions5.rs
//
// Execute `rustlings hint functions5` or use the `hint` watch subcommand for a
// hint.

fn main() {
    let answer: i32 = square(3);
    println!("The square of 3 is {}", answer);
}

fn square(num: i32) -> i32 {
    // You could do the following:
    // let newnum: i32 = num * num;
    // newnum
    // Here, newnum is returned implicitly and has the expected return type.

    // However, Rust gives you an explicit return statement which achieves the
    // ... same result, but in fewer lines of code.
    // Note that you don't have to create a temporary variable to store the result
    // ..., the return statement handles this for you.
    return num * num;
}
