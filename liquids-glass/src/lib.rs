use std::collections::HashMap;

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
