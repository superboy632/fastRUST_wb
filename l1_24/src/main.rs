use std::collections::HashSet;
fn has_unique_characters(input: &str) -> bool {
    let mut chars_set = HashSet::new();

    for ch in input.to_lowercase().chars() {
        if !chars_set.insert(ch) {
            return false; // Если символ уже существует в сет, значит есть дубликат
        }
    }

    true
}

fn main() {
    let test_strings = vec!["abcd", "abCdefAaf", "aabcd"];

    for s in test_strings {
        println!("\"{}\" has unique characters: {}", s, has_unique_characters(s));
    }
}
