// move_semantics1.rs
//
// Execute `rustlings hint move_semantics1` or use the `hint` watch subcommand
// for a hint.

#[test]
fn main() {
    let vec0 = vec![22, 44, 66];

    let vec1 = fill_vec(vec0);

    assert_eq!(vec1, vec![22, 44, 66, 88]);
}

fn fill_vec(vec: Vec<i32>) -> Vec<i32> {
    // Create a new empty vector
    let mut new_vec: Vec<i32> = Vec::new();
    // Loop over elements of vec, dereference, and push to new_vec
    for num in &vec {
        new_vec.push(*num);
    }
    // Push 88 to the end of new_vec before returning
    new_vec.push(88);
    // Return new_vec
    new_vec
}
