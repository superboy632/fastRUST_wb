use std::io;
use std::io::BufRead;

fn main() {
    let mut unique_strings = Vec::new();
    let stdin = io::stdin();
    //читаем входные строки
    // Внмание: цикл ввода будет бесконечный, пока не нажмете ctrl+z(windows), ctrl+d(linux) или command+d(macoc)
    for line in stdin.lock().lines() {
        let input = line.unwrap();
        if !unique_strings.contains(&input) {
            unique_strings.push(input); // проверяем была ли уже эта срока и добвляем, если она уникальна
        }
    }
    println!("Уникальные строки:");
    for strings in unique_strings {
        println!("\t{} ", strings);
    }
}
