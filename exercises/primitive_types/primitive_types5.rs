// primitive_types5.rs
//
// Destructure the `cat` tuple so that the println will work.
//
// Execute `rustlings hint primitive_types5` or use the `hint` watch subcommand
// for a hint.

fn main() {
    let cat: (&str, f64) = ("Furry McFurson", 3.5);
    // Don't need to borrow from tuple because it is a 'compounded' element
    // Therefore we can unpack it directly to new variables.

    // Solution 1 - verbose tuple unpacking

    // let name: &str = cat.0;
    // let age: f64 = cat.1;

    // Solution 2 - concise tuple unpacking
    let (name, age) = cat;

    println!("{} is {} years old.", name, age);
}
