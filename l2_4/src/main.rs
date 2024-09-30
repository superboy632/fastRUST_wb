/*
Поиск анаграмм по словарю
Написать функцию поиска всех множеств анаграмм по словарю.
Например:

'пятак', 'пятка' и 'тяпка' — принадлежат одному множеству,

'листок', 'слиток' и 'столик' — другому.
*/
use std::collections::{HashMap, HashSet};

fn get_letter_stat(line: &[&str]) -> HashMap<String, Vec<String>> {
    let mut stat:HashMap<String, HashSet<String>> = HashMap::new();
    for  word in line.iter() {
        let sorted_word: String = {
            let mut chars: Vec<_> = word.to_lowercase().chars().collect();
            chars.sort();
            chars.into_iter().collect::<String>()
        };
        stat.entry(sorted_word).or_insert_with(HashSet::new).insert(word.to_lowercase());
    }
    stat
        .into_iter()
        .filter_map(|(_, words_set)| {
            if words_set.len() > 1 {
                let mut sorted_words: Vec<String> = words_set.into_iter().collect();
                sorted_words.sort();
                Some((sorted_words[0].clone(), sorted_words)) // возвращаем в хешмапу первое слово анаграмы и множетсво
            } else {
                None
            }
        })
        .collect()
}

fn main() {
    let array = ["Пятак", "пятка", "тяпка", "листок", "слиток", "столик"];
    let anagrams = get_letter_stat(&array);

    for (key, value) in anagrams {
        println!("key:{key}, value:{:?}", value);
    }
 }

// #[cfg(test)]
// mod tests {
//     use crate::{get_letter_stat};
//
//     #[test]
//     fn case_main() {
//         let array = ["пятак", "пятка", "тяпка", "листок", "слиток", "столик"];
//         let result = ["пятак", "пятка", "тяпка"];
//         assert_eq!(result, get_letter_stat(&array));
//     }
// }