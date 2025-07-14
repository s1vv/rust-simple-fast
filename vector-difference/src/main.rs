use std::collections::HashSet;

/// Функция принимает два вектора и возвращает вектор со значениями из первого
/// которых нет во втором векторе.
fn select_unmatched<T>(v1: &[T], v2: &[T]) -> Vec<T>
where
    T: Eq + Copy + std::hash::Hash,
{
    let mut res: Vec<T> = vec![];
    let set: HashSet<_> = v2.into_iter().collect();
    for num in v1 {
        if !set.contains(num) {
            res.push(*num);
        }
    }
    res
}

fn main() {
    let vec1 = vec![1, 2, 5, 8];
    let vec2 = vec![5, 8, 5, 11];
    dbg!(select_unmatched(&vec1, &vec2));
    let vec1 = vec!["ab", "cd", "cc"];
    let vec2 = vec!["cd", "tt"];
    dbg!(select_unmatched(&vec1, &vec2));
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_select_unmatched() {
        let vec1 = vec![1, 2, 5, 8];
        let vec2 = vec![5, 8, 5, 11];
        assert_eq!(select_unmatched(&vec1, &vec2), vec![1, 2]);
        let vec1 = vec!["ab", "cd", "cc"];
        let vec2 = vec!["cd", "tt"];
        assert_eq!(select_unmatched(&vec1, &vec2), vec!["ab", "cc"]);
    }
}
