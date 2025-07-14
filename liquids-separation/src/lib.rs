use std::collections::HashMap;

/// Принимает ссылку на двумерный массив символов и возвращает отсортированный
/// массив.
pub fn separate_liquids(glass: &[Vec<char>]) -> Vec<Vec<char>> {
    if glass.is_empty() {
        return Vec::new();
    }

    let weight = HashMap::from([('H', 1.36), ('W', 1.00), ('A', 0.87), ('O', 0.8)]);
    let glass_width = glass[0].len();

    let mut liquids: Vec<char> = glass.iter().flat_map(|row| row.iter().copied()).collect();

    liquids.sort_by(|c1, c2| {
        let w1 = weight.get(c1).unwrap_or(&0.0);
        let w2 = weight.get(c2).unwrap_or(&0.0);
        w1.partial_cmp(w2).unwrap()
    });

    liquids
        .chunks(glass_width)
        .map(|chunk| chunk.to_vec())
        .collect()
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
