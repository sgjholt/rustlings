// options2.rs
//
// Execute `rustlings hint options2` or use the `hint` watch subcommand for a
// hint.

#[cfg(test)]
mod tests {
    #[test]
    fn simple_option() {
        let target = "rustlings";
        let optional_target = Some(target);

        if let Some(word) = optional_target {
            assert_eq!(word, target);
        }
    }

    #[test]
    fn layered_option() {
        let range = 10;
        let mut optional_integers: Vec<Option<i8>> = vec![None];

        for i in 1..(range + 1) {
            optional_integers.push(Some(i));
        }

        let mut cursor = range;

        // optional integers: Vec<Some(x1), Some(x2), ... Some(xn)>
        // While let adds one more layer of Some to the .pop
        // So to make integer equivalent you need to wrap in two Sums
        while let Some(Some(integer)) = optional_integers.pop() {
            // The comparison happens and then the values are returned without
            // being wrapped in Some once the Some check is finished
            assert_eq!(integer, cursor);
            // Now cursor is just an i8, so we can subtract an int from it
            cursor -= 1;
        }
        // At the end, cursor should be 0 because we subtract 1 every time
        // we go through an iteration of the loop until we've got nothing left.
        assert_eq!(cursor, 0);
    }
}
