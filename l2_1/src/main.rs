use std::{env, io, process};
use std::fs::File;
use std::io::{Write, BufReader, BufRead, Error};

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

fn count_of_words(config: Config) -> usize {
    let file = File::open(&config.filename).unwrap_or_else(|_| {
        eprintln!("Не удалось открыть файл: {}", &config.filename);
        process::exit(1);
    });
    let reader = BufReader::new(file);
    reader
        .lines()
        .filter_map(|result| result.ok()) // игнорируем возможные ошибки чтения
        .flat_map(|line| line.split_whitespace()) // разбиваем строки на слова
        .count()
}


fn main() {
    let config = get_file_name(env::args()).unwrap_or_else(|err| {
       eprintln!("fail to read a command line: {err}");
        process::exit(1);
    });

    let count = match config.flag.as_str() {
        // "-l" => count_lines(&file),
        "-w" => count_of_words(config),
        // "-c" => count_chars(&file),
        _ => count_of_words(config),
    };

}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn case_count() {
        let config: Config = Config {filename: "test.txt".parse().unwrap(), flag: "-w".parse().unwrap() };
        assert_eq!(15, count_of_words(config));
    }
}
