// variables5.rs
//
// Execute `rustlings hint variables5` or use the `hint` watch subcommand for a
// hint.

fn main() {
    let mut number: &str = "T-H-R-E-E"; // don't change this line
    println!("Spell a Number : {}", number);
    // Shadowing example - I can create a temporary scope where number is
    // ... created as a new variable and can have any type it wants.
    {
        let number: i32 = 3; // don't rename this variable
        println!("Number plus two is : {}", number + 2);
    }
}
