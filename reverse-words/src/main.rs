use reverse_words::{reverse_words_fast, reverse_words_slow};

fn main() {
    let my_string = "Rust blazing fast";
    dbg!(reverse_words_slow(my_string));
    dbg!(reverse_words_fast(my_string));
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_reverse_words_slow() {
        assert_eq!(
            reverse_words_slow("Rust blazing fast 12345"),
            "Rust gnizalb fast 54321".to_string()
        )
    }

    #[test]
    fn test_reverse_words_fast() {
        assert_eq!(
            reverse_words_fast("Rust blazing fast 12345"),
            "Rust gnizalb fast 54321".to_string()
        )
    }
}
