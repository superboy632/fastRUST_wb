use std::error::Error;
use std::fs;
use clap::Parser;

//Box<dyn Error> - позволяет вернуть разные типы ошибок
pub fn run(config: Config) -> Result<(), Box<dyn Error>>{
    let contents = fs::read_to_string(config.files)?;


    let results = if config.ignore_case {
        search_case_insensitive(&config.pattern, &contents)
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
    #[arg(short = 'A')]
    pub after: bool,

    /// печатать +N строк до совпадения
    #[arg(short = 'B')]
    pub before: bool,

    ///  (A+B) печатать ±N строк вокруг совпадения
    #[arg(short = 'C')]
    pub context: bool,

    /// количество строк
    #[arg(short = 'c')]
    pub count: bool,

    /// (вместо совпадения, исключать)
    #[arg(short = 'v')]
    pub invert: bool,

    /// точное совпадение со строкой, не паттерн
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

// pub fn search_case_after<'a> (query: &str, contents: &'a str, mut n: i32) -> Vec<&'a str> {
//     let mut results = Vec::new();
//     for line in contents.lines() {
//         if line.contains(query) {
//             results.push(line);
//             while n > 0 {
//                 results.push(line);
//                 if line.next().is_none() {
//                     break;
//                 }
//                 n -= 1;
//             }
//         }
//     }
//     results
// }

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

//     #[test]
//     fn case_intensive() {
//         let n = 1;
//         let query = "Rust";
//         let contents = "\
// Rust:
// safe, fast, productive.
// Trust me.
// Duct tape.";
//         assert_eq!(vec!["Rust:", "safe, fast, productive."],
//                    search_case_after(query, contents, n));
//     }
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
}