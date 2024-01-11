// iterators2.rs
//
// In this exercise, you'll learn some of the unique advantages that iterators
// can offer. Follow the steps to complete the exercise.
//
// Execute `rustlings hint iterators2` or use the `hint` watch subcommand for a
// hint.

// Step 1.
// Complete the `capitalize_first` function.
// "hello" -> "Hello"
pub fn capitalize_first(input: &str) -> String {
    // Convert input into array of chars like: ["h", "e", "l", "l", "o"]
    let mut c = input.chars();
    // Match the first char yielded by iterator created from call: c.next()
    let cap_first = match c.next() {
        // Case 1: Iterator yields None (str was empty | "")
        // Create a new empty string.
        None => String::new(),
        // Case 2: Iterator yields char
        // Convert char to string -> uppercase -> add rest of string via slice.
        Some(first) => String::from(first).to_uppercase() + &input[1..],
    };
    // return whatever is assigned by match
    cap_first
}

// Step 2.
// Apply the `capitalize_first` function to a slice of string slices.
// Return a vector of strings.
// ["hello", "world"] -> ["Hello", "World"]
pub fn capitalize_words_vector(words: &[&str]) -> Vec<String> {
    let mut cap_words_vec: Vec<String> = Vec::new();
    for word in words.iter() {
        cap_words_vec.push(capitalize_first(word));
    }
    cap_words_vec
}

// Step 3.
// Apply the `capitalize_first` function again to a slice of string slices.
// Return a single string.
// ["hello", " ", "world"] -> "Hello World"
pub fn capitalize_words_string(words: &[&str]) -> String {
    let mut all_words: String = String::new();
    for word in words.iter() {
        all_words += &capitalize_first(word);
    }
    all_words
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_success() {
        assert_eq!(capitalize_first("hello"), "Hello");
    }

    #[test]
    fn test_empty() {
        assert_eq!(capitalize_first(""), "");
    }

    #[test]
    fn test_iterate_string_vec() {
        let words = vec!["hello", "world"];
        assert_eq!(capitalize_words_vector(&words), ["Hello", "World"]);
    }

    #[test]
    fn test_iterate_into_string() {
        let words = vec!["hello", " ", "world"];
        assert_eq!(capitalize_words_string(&words), "Hello World");
    }
}
