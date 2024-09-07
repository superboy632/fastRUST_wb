use std::thread;

fn main() {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).expect("can't read line");
    let n= input.trim().parse().expect("Incorrect number");

    // обрабатываем введеное число на очень большое число и отрицательное
    if n > 20000 || n < 0 {
        eprintln!("Недопустимый диапазон занчений");
        std::process::exit(1);
    }

    let numbers: Vec<i32> = (1..=n).collect(); // Инициализация массива 1..N

    let mut handles = vec![];

    for &number in &numbers {
        // Создаём новый поток для каждого числа
        let handle = thread::spawn(move || { //с помощью move заставляем замыкание забирать во владение используемые значения
            let square = number * number; // Вычисление квадрата
            square // Возвращаем результат
        });
        handles.push(handle);
    }

    // Сбор результатов
    let mut results = vec![];
    for handle in handles {
        // Получаем результат из потока
        if let Ok(result) = handle.join() {
            results.push(result);
        }
    }

    // Вывод результатов в stdout
    for result in results {
        println!("{}", result);
    }
}