// quiz2.rs
//
// This is a quiz for the following sections:
// - Strings
// - Vecs
// - Move semantics
// - Modules
// - Enums
//
// Let's build a little machine in the form of a function. As input, we're going
// to give a list of strings and commands. These commands determine what action
// is going to be applied to the string. It can either be:
// - Uppercase the string
// - Trim the string
// - Append "bar" to the string a specified amount of times
// The exact form of this will be:
// - The input is going to be a Vector of a 2-length tuple,
//   the first element is the string, the second one is the command.
// - The output element is going to be a Vector of strings.
//
// No hints this time!

pub enum Command {
    Uppercase,
    Trim,
    Append(usize),
}

mod my_module {
    use std::ascii::AsciiExt;

    use super::Command;

    // Takes Vec of tuples of (String, Command) outputs Vec of Strings
    pub fn transformer(input: Vec<(String, Command)>) -> Vec<String> {
        // Define a mutable vec object to insert strings into
        let mut output: Vec<String> = vec![];
        // Loop through Vec, and unpack tuples into variable refs
        for (string, command) in input.iter() {
            // Match the command, perform corresponding operation, and return to s
            let s: String = match command {
                Command::Uppercase => string.to_uppercase(), // to_uppercase() converts to String from &str
                Command::Trim => string.trim().to_string(),
                Command::Append(n) => string.to_string() + &"bar".repeat(*n),
            };
            // Push s: String to output: Vec<String>
            output.push(s)
        }
        // Return output Vec<String>
        output
    }
}

#[cfg(test)]
mod tests {
    use super::my_module::transformer;
    use super::Command;

    #[test]
    fn it_works() {
        let output = transformer(vec![
            ("hello".into(), Command::Uppercase),
            (" all roads lead to rome! ".into(), Command::Trim),
            ("foo".into(), Command::Append(1)),
            ("bar".into(), Command::Append(5)),
        ]);
        assert_eq!(output[0], "HELLO");
        assert_eq!(output[1], "all roads lead to rome!");
        assert_eq!(output[2], "foobar");
        assert_eq!(output[3], "barbarbarbarbarbar");
    }
}
