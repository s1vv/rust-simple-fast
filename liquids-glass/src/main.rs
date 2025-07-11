use liquids_glass::separate_liquids;

///
fn main() {
    let mut glass1 = vec![
        vec!['H', 'H', 'W', 'O'],
        vec!['W', 'W', 'O', 'W'],
        vec!['H', 'H', 'O', 'O'],
    ];
    dbg!(separate_liquids(&mut glass1));
}

#[cfg(test)]
mod tests {
    use std::vec;

    use super::*;

    const ERR_MSG: &str = "\nYour result (left) did not match the expected output (right)";

    fn dotest(a: &[Vec<char>], expected: &[Vec<char>]) {
        assert_eq!(
            separate_liquids(a),
            expected,
            "{ERR_MSG} with glass = {a:?}"
        )
    }

    #[test]
    fn fixed_tests() {
        dotest(
            &[
                vec!['H', 'H', 'W', 'O'],
                vec!['W', 'W', 'O', 'W'],
                vec!['H', 'H', 'O', 'O'],
            ],
            &[
                vec!['O', 'O', 'O', 'O'],
                vec!['W', 'W', 'W', 'W'],
                vec!['H', 'H', 'H', 'H'],
            ],
        );
        dotest(
            &[
                vec!['A', 'A', 'O', 'H'],
                vec!['A', 'H', 'W', 'O'],
                vec!['W', 'W', 'A', 'W'],
                vec!['H', 'H', 'O', 'O'],
            ],
            &[
                vec!['O', 'O', 'O', 'O'],
                vec!['A', 'A', 'A', 'A'],
                vec!['W', 'W', 'W', 'W'],
                vec!['H', 'H', 'H', 'H'],
            ],
        );
        dotest(&[vec!['A', 'H', 'W', 'O']], &[vec!['O', 'A', 'W', 'H']]);
        dotest(
            &[vec!['A'], vec!['H'], vec!['W'], vec!['O']],
            &[vec!['O'], vec!['A'], vec!['W'], vec!['H']],
        );
        dotest(&[], &[]);
        dotest(
            &[
                vec!['A', 'O', 'W', 'W', 'W', 'A', 'O'],
                vec!['H', 'O', 'H', 'H', 'A', 'A', 'W'],
                vec!['A', 'A', 'W', 'O', 'H', 'A', 'W'],
                vec!['H', 'H', 'A', 'W', 'O', 'W', 'H'],
                vec!['H', 'O', 'H', 'A', 'A', 'O', 'O'],
                vec!['W', 'W', 'H', 'W', 'A', 'A', 'A'],
            ],
            &[
                vec!['O', 'O', 'O', 'O', 'O', 'O', 'O'],
                vec!['O', 'A', 'A', 'A', 'A', 'A', 'A'],
                vec!['A', 'A', 'A', 'A', 'A', 'A', 'A'],
                vec!['W', 'W', 'W', 'W', 'W', 'W', 'W'],
                vec!['W', 'W', 'W', 'W', 'H', 'H', 'H'],
                vec!['H', 'H', 'H', 'H', 'H', 'H', 'H'],
            ],
        );
    }
}
