fn reverse_str(input: &str) -> String {
    let words: Vec<&str> = input.split_whitespace().collect(); // строку из слов делим на слова, каждое слово помещая в массив
    let reverse_words: Vec<&str> = words.iter().rev().map(|&word| word).collect(); // меняем порядок слов в строке наоборот
    reverse_words.join(" ") // добавляем разделители
}

fn main() {
    let input: &str = "test join rust";
    let result = reverse_str(&input);
    println!("{:?}", result);
}
