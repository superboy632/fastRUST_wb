use std::{env, process};

use l2_5::Config;


fn main() {
    // принимаем аргументы командной строки и сразу вызываем метод collect для преобразования итератора в вектор
    let args: Vec<String> = env::args().collect();

    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("Problems with parsing arguments: {}", err);
        process::exit(1); // немедленно останавливает программу и передает код ошибки
    });

    if let Err(e) = l2_5::run(config) {
        println!("Application error: {e}");
        process::exit(1);
    }
}