// functions3.rs
//
// Execute `rustlings hint functions3` or use the `hint` watch subcommand for a
// hint.

fn main() {
    // u32 is an 'unsigned' integer -> it cannot be negative
    // If you replace '3' with '-3' this code will break
    let num: u32 = 3;
    call_me(num);
}

fn call_me(num: u32) {
    // The syntax is for <var> in <start>..<end> {do thing;}
    // The grey text in the middle in VS Code is inserted to show you what is
    // ... actually being called.
    for i in 0..num {
        println!("Ring! Call number {}", i + 1);
    }
}
