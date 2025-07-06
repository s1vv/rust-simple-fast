use core::convert::From;
use std::collections::HashMap;

/// Создать константу с "жидкостями", в функции константу сортировать, может быть использовать map_flat
fn separate_liquids(glass: &[Vec<char>]) -> Vec<Vec<char>> {
    let liquids: HashMap<&str, f64> =
        HashMap::from([("H", 1.36), ("W", 1.00), ("A", 0.87), ("O", 0.80)]);
}

fn main() {
    let glass = [
        vec!['H', 'H', 'W', 'O'],
        vec!['W', 'W', 'O', 'W'],
        vec!['H', 'H', 'O', 'O'],
    ];
    dbg!(separate_liquids(&glass));
}

// Add your tests here.
// See https://doc.rust-lang.org/stable/rust-by-example/testing/unit_testing.html

#[cfg(test)]
mod tests {
    use super::remove_exclamation_marks;

    fn do_test(input: &str, expected: &str) {
        let actual = remove_exclamation_marks(input);
        assert_eq!(
            actual, expected,
            "\nYour result (left) did not match the expected output (right) for the input: {input:?}"
        );
    }

    #[test]
    fn sample_test() {
        do_test("Hello World!", "Hello World");
        do_test("Hello World!!!", "Hello World");
        do_test("Hi! Hello!", "Hi Hello");
        do_test("", "");
        do_test("Oh, no!!!", "Oh, no");
    }
}
