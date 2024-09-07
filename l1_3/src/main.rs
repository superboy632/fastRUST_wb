use std::thread;
use std::sync::mpsc;
use std::time::Duration;

fn main() {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).expect("can't read line");
    let n= input.trim().parse().expect("Incorrect number");

    // обрабатываем введеное число на очень большое число и отрицательное
    if n > 1800 || n < 0 {
        eprintln!("Недопустимый диапазон занчений");
        std::process::exit(1);
    }

    let numbers: Vec<i32> = (1..=n).collect(); // Инициализация массива 1..N

    let (tx, rx) = mpsc::channel();

    let handle = thread::spawn(move || {
       for &numbers in &numbers {
           let square = numbers*numbers;
           tx.send(square).unwrap();
           thread::sleep(Duration::from_micros(1));
       }
    });

    // Сбор результатов
    let mut result = 0;
    for received in rx {
        result += received;
    }

    handle.join().unwrap();

    // Вывод результатов в stdout
    println!("{}", result);
}
