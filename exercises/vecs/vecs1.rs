// vecs1.rs
//
// Your task is to create a `Vec` which holds the exact same elements as in the
// array `a`.
//
// Make me compile and pass the test!
//
// Execute `rustlings hint vecs1` or use the `hint` watch subcommand for a hint.

fn array_and_vec() -> ([i32; 4], Vec<i32>) {
    let a = [10, 20, 30, 40]; // a plain array

    // Solution 1 - using self.to_vec() - creates a copy into a new vector so you
    // ... don't need to borrow from a.
    // let v: Vec<i32> = a.to_vec();

    // Solution 2 - long format
    // Initialize and empty mutable Vec
    let mut v: Vec<i32> = Vec::new();
    // Loop over borrowed elements of a and borrow over to i: &i32
    for i in &a {
        // Dereference i (&i32) from a (i32) so (&i32 -> i32) and
        v.push(*i); // append i to v (mutable Vec<i32>)
    }
    // Place them into a Tuple before returning
    (a, v)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_array_and_vec_similarity() {
        let (a, v) = array_and_vec();
        assert_eq!(a, v[..]);
    }
}
