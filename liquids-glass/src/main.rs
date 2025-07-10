use std::collections::HashMap;

fn separate_liquids(glass: &[Vec<char>]) -> Vec<Vec<char>> {
    if glass.is_empty() {
        Vec::new()
    } else {
        let weight = HashMap::from([('H', 1.36), ('W', 1.00), ('A', 0.87), ('O', 0.8)]);
        let mut glass_sorted = glass.to_vec();
        if glass.len() == 1 {
            glass_sorted[0].sort_by(|k1, k2| {
                let l1 = weight.get(k1).unwrap_or(&0.0);
                let l2 = weight.get(k2).unwrap_or(&0.0);
                l1.partial_cmp(l2).unwrap()
            });
        }

        let glass_width = glass[0].len();

        let mut liquids_count: HashMap<char, u32> = HashMap::new();

        // только читаем, не меняем glass
        for v in glass.iter() {
            for c in v.iter() {
                *liquids_count.entry(*c).or_insert(0) += 1;
            }
        }

        let mut liquids_count = liquids_count.into_iter().collect::<Vec<_>>();
        liquids_count.sort_by(|(k1, _), (k2, _)| {
            let l1 = weight.get(k1).unwrap_or(&0.0);
            let l2 = weight.get(k2).unwrap_or(&0.0);
            l1.partial_cmp(l2).unwrap()
        });

        let error = 'E';

        // теперь второй цикл - уже можем мутировать glass
        let mut index: usize = 0;
        for (ch, count) in liquids_count {
            for _ in 0..count as usize / glass_width {
                glass_sorted[index] = match ch {
                    'H' => vec!['H'; glass_width],
                    'O' => vec!['O'; glass_width],
                    'A' => vec!['A'; glass_width],
                    'W' => vec!['W'; glass_width],
                    _ => vec![error; glass_width],
                };
                index += 1;
            }
        }

        glass_sorted
    }
}

///
fn main() {
    let mut glass2 = vec![vec!['A', 'H', 'W', 'O']];
    dbg!(separate_liquids(&mut glass2));
}

#[cfg(test)]
mod tests {
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
    }
}
