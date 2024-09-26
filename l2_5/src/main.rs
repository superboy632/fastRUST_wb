use std::{process};
use clap::{Parser};

use l2_5::Config;



fn main() {

    // принимаем аргументы командной строки и сразу вызываем метод collect для преобразования итератора в вектор
    let args = Config::parse();


    if let Err(e) = l2_5::run(args) {
        println!("Application error: {e}");
        process::exit(1);
    }
}