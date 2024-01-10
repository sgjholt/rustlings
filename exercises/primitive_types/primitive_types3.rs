// primitive_types3.rs
//
// Create an array with at least 100 elements in it where the ??? is.
//
// Execute `rustlings hint primitive_types3` or use the `hint` watch subcommand
// for a hint.

fn main() {
    // The repeat method reapeats the string n:(uint32) times.
    // I repeated the short string long enough to exceed 100 array items.
    let a: String = "Man I love 'da cake!\n".repeat(100);
    if a.len() >= 100 {
        // println!("{a}"); // Uncomment if you want to see what it is doing.
        println!("Wow, that's a big array!");
    } else {
        println!("Meh, I eat arrays like that for breakfast.");
        panic!("Array not big enough, more elements needed")
    }
}
