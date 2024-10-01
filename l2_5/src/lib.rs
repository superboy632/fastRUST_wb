use std::error::Error;
use std::{fs};
use clap::Parser;

//Box<dyn Error> - позволяет вернуть разные типы ошибок
pub fn run(config: Config) -> Result<(), Box<dyn Error>>{
    let contents = fs::read_to_string(config.files)?;

    let results = if config.ignore_case {
        search_case_insensitive(&config.pattern, &contents)
    } else if config.fixed {
        search_case_fixed(&config.pattern, &contents)
    } else if config.invert {
        search_case_kill(&config.pattern, &contents)
    } else if config.after.is_some() {
        let n = config.after.unwrap();
        search_case_after(&config.pattern, &contents, n)
    } else if config.context.is_some() {
        let n = config.context.unwrap();
        search_case_after_before(&config.pattern, &contents, n)
    } else if config.before.is_some() {
        let n = config.before.unwrap();
        search_case_before(&config.pattern, &contents, n)
    } else {
        search(&config.pattern, &contents)
    };

    if config.count {
        let count = search_case_count(&config.pattern, &contents);
        println!("{count}");
    };

    for (line_number, line) in results.into_iter().enumerate() {
        if config.line_number {
            println!("{}:{line}", line_number + 1);
        } else if !config.count {
            println!("{line}");
        }
    }
    Ok(())
}
#[derive(Parser)]
pub struct Config {
    /// Паттерн для поиска
    pub pattern: String,

    /// Файлы для поиска
    pub files: String,

    /// Игнорировать регистры
    #[arg(short = 'i')]
    pub ignore_case: bool,

    /// Выводить номер строки
    #[arg(short = 'n')]
    pub line_number: bool,

    /// печатать +N строк после совпадения
    #[arg(short = 'A', value_parser = clap::value_parser!(usize))]
    pub after: Option<usize>,

    /// печатать +N строк до совпадения
    #[arg(short = 'B', value_parser = clap::value_parser!(usize))]
    pub before: Option<usize>,

    ///  (A+B) печатать ±N строк вокруг совпадения
    #[arg(short = 'C', value_parser = clap::value_parser!(usize))]
    pub context: Option<usize>,

    /// количество строк
    #[arg(short = 'c')]
    pub count: bool,

    /// (вместо совпадения, исключать)
    #[arg(short = 'v')]
    pub invert: bool,

    /// точное совпадение со строкой, не паттерн. ИНСТРУКЦИЯ: для использования этого флага строку
    /// следует вводить следующим образом: "How public, like a frog"
    #[arg(short = 'F')]
    pub fixed: bool,
}

pub fn search<'a> (query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();
    for line in contents.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }
    results
}

pub fn search_case_insensitive<'a> (
    query: &str,
    contents: &'a str
) -> Vec<&'a str> {
    let query = query.to_lowercase();
    let mut results = Vec::new();
    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            results.push(line);
        }
    }
    results
}

pub fn search_case_count(
    query: &str,
    contents: &str
) -> i32 {
    let mut results = 0;
    for line in contents.lines() {
        if line.contains(&query) {
            results += 1;
        }
    }
    results
}


pub fn search_case_fixed<'a> (
    query: &str,
    contents: &'a str
) -> Vec<&'a str>  {
    let mut results = Vec::new();
    for line in contents.lines() {
        if line.eq(query) {
            results.push(line);
        }
    }
    results
}

pub fn search_case_kill<'a> (
    query: &str,
    contents: &'a str
) -> Vec<&'a str>  {
    let mut results = Vec::new();
    for line in contents.lines() {
        if !line.contains(&query) {
            results.push(line);
        }
    }
    results
}
pub fn search_case_after_before<'a> (query: &str, contents: &'a str, n: usize) -> Vec<&'a str> {
    let mut results = Vec::new();
    let results1 = search_case_before(query, contents, n);
    let results2 = search_case_after(query, contents, n);
    for line in 0..results1.iter().count() - 1 {
        results.push(results1[line])
    }
    for line in results2 {
        results.push(line)
    }
    results
}
pub fn search_case_after<'a> (query: &str, contents: &'a str, mut n: usize) -> Vec<&'a str> {
    let mut results = Vec::new();
    let mut flag:bool = false;
    for line in contents.lines() {
        if flag.eq(&true) && n > 0 {
            results.push(line);
            n -= 1;
        }
        if line.contains(query) {
            flag = true;
            results.push(line);
        }
    }
    results
}

pub fn search_case_before<'a> (query: &str, contents: &'a str, n: usize) -> Vec<&'a str> {
    let mut results = Vec::new();
    let mut flag:bool = false;
    let mut count = 0;
    if n > contents.lines().count() {
        panic!("wrong N");
    }
    for line in contents.lines() {
        if line.contains(query) {
            flag = true;
            for _ in 0..(count - n) {
                results.remove(0);
            }
            results.push(line);
        }
        if flag.eq(&false) {
            results.push(line);
            count += 1;
        }
    }
    results
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";
        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }
    #[test]
    fn case_intensive() {
        let query = "rUst";
        let contents = "\
Rust:
safe, fast, productive.
Trust me.";
        assert_eq!(vec!["Rust:", "Trust me."],
                   search_case_insensitive(query, contents));
    }

    #[test]
    fn case_after() {
        let n = 1;
        let query = "Rust";
        let contents = "\
Rust:
safe, fast, productive.
Trust me.
Duct tape.";
        assert_eq!(vec!["Rust:", "safe, fast, productive."],
                   search_case_after(query, contents, n));
    }
    #[test]
    fn case_count() {
        let query = "Rust";
        let contents = "\
    Rust:
    Rust
    safe, fast, productive.
    Trust me.";
        assert_eq!(2, search_case_count(query, contents));
    }

    #[test]
    fn case_fixed() {
        let query = "Rust";
        let contents = "\
    Rust:
Rust
    safe, fast, productive.
    Trust me.";
        assert_eq!(vec!["Rust"], search_case_fixed(query, contents));
    }
    #[test]
    fn case_kill() {
        let query = "Rust";
        let contents = "\
Rust:
Rust
safe, fast, productive.
Trust me.";
        assert_eq!(vec!["safe, fast, productive.", "Trust me."], search_case_kill(query, contents));
    }
    #[test]
    fn case_before() {
        let n = 1;
        let query = "How public, like a frog";
        let contents = "\
I'm nobody! Who are you?
Are you nobody, too?
Then there's a pair of us - don't tell!
They'd banish us, you know.

How dreary to be somebody!
How public, like a frog
To tell your name the livelong day
To an admiring bog!";
        assert_eq!(vec!["How dreary to be somebody!", "How public, like a frog"],
                   search_case_before(query, contents, n));
    }
}