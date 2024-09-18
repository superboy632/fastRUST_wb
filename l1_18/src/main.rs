
fn reverse(input: &str) -> String {
    let string_unicode: Vec<char> = input.chars().collect(); // строку преобразуем в вектор символов для работы с Unicode
    string_unicode.into_iter().rev().collect() // переворачиваем вектор и собираем в строку
}


fn main() {
    let input: &str = "testffhgfjhgdk";
    let result = reverse(&input);
    println!("{:?}", result);
}
