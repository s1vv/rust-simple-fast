use liquids_separation::separate_liquids;

///
fn main() {
    let mut glass1 = vec![vec!['H', 'H', 'W', 'O']];
    dbg!(separate_liquids(&mut glass1) == vec![vec!['O', 'W', 'H', 'H']]);
}
