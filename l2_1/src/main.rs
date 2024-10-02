use std::{env, process};
use std::fs::{read_to_string};

struct Config {
    filename: String,
    flag: String,
}


fn get_file_name<'a>(mut args: impl Iterator<Item = String>) -> Result<Config, &'static str> {
    if args.next().is_none() {
        eprintln!("no arguments");
        process::exit(1);
    }

    let buf = match args.next() {
        Some(arg) => arg,
        None => return Err("no arguments"),
    };

    let config: Config = if !buf.starts_with('-') {
        Config {filename: buf, flag: "-w".parse().unwrap() }
    } else {
        let second_arg = match args.next() {
            Some(arg) => arg,
            None => return Err("no arguments"),
        };
        Config {filename: second_arg, flag: buf}
    };

    Ok(config)
}

fn count_lines(config: Config) -> usize {
    let contents = read_to_string(config.filename);
    let lines = contents.unwrap().lines().count();
    lines
}

fn count_chars(config: Config) -> usize {
    let contents = read_to_string(config.filename);
    let chars = contents.unwrap().chars().count();
    chars
}

fn count_words(config: Config) -> usize {
    let contents = read_to_string(config.filename);
    let words = contents.unwrap().split_whitespace().count();
    words
}


fn main() {
    let config = get_file_name(env::args()).unwrap_or_else(|err| {
       eprintln!("fail to read a command line: {err}");
        process::exit(1);
    });

    let count = match config.flag.as_str() {
        "-l" => count_lines(config),
        "-w" => count_words(config),
        "-c" => count_chars(config),
        _ => count_words(config),
    };

    println!("{}", count);

}