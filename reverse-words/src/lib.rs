/// Напишите функцию, которая принимает строку из одного или нескольких слов и
/// возвращает ту же строку, но все слова, содержащие пять или более букв,
/// меняются местами (как в названии этого ката). Передаваемые строки будут
/// состоять только из букв и пробелов. Пробелы будут включены только в том
/// случае, если присутствует более одного слова.
pub fn reverse_words_slow(s: &str) -> String {
    s.split_whitespace()
        .map(|w| {
            if w.len() >= 5 {
                w.chars().rev().collect::<String>()
            } else {
                w.to_string()
            }
        })
        .collect::<Vec<String>>()
        .join(" ")
}

pub fn reverse_words_fast(s: &str) -> String {
    let mut res = String::with_capacity(s.len());
    for w in s.split_whitespace() {
        if w.len() >= 5 {
            res.extend(w.chars().rev());
        } else {
            res.push_str(w);
        }
        res.push(' ');
    }
    res.pop();
    res
}
