use std::collections::HashSet;

fn vector_difference(v1: &[i32], v2: &[i32]) -> Vec<i32> {
    let set: HashSet<_> = v2.into_iter().collect();
    let mut res: Vec<i32> = vec![];
    for num in v1 {
        if set.contains(num) {
            continue;
        } else {
            res.push(*num);
        }
    }
    res
}
fn main() {
    let vec1 = vec![1, 2, 5, 8, 7];
    let vec2 = vec![5, 8, 5, 11, 7];
    dbg!(vector_difference(&vec1, &vec2));
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_vector_difference() {
        let vec1 = vec![1, 2, 5, 8];
        let vec2 = vec![5, 8, 5, 11];
        assert_eq!(vector_difference(&vec1, &vec2), vec![1, 2])
    }
}
