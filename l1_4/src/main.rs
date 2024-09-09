use std::sync::{mpsc, Arc, Mutex};
use std::thread;
use std::time::Duration;
use tokio::signal::ctrl_c;

#[tokio::main]
async fn main() {
    let (tx, rx) = mpsc::channel();
    let rx = Arc::new(Mutex::new(rx)); //

    // Получаем количество воркеров из аргументов командной строки
    // let num_workers: usize = std::env::args()
    //     .nth(1)
    //     .unwrap_or_else(|| "4".to_string()) // Значение по умолчанию 4
    //     .parse()
    //     .expect("Please provide a valid number of workers");

    let mut num_workers = String::new();
    std::io::stdin().read_line(&mut num_workers).expect("can't read line");
    let n = num_workers.trim().parse().expect("Incorrect number");

    if  n < 0 {
        eprintln!("Недопустимый диапазон занчений");
        std::process::exit(1);
    }

    // Запускаем воркеры
    for i in 0..n {
        let rx1 = Arc::clone(&rx);
        thread::spawn(move || {
            loop {
                let rx = rx1.lock().unwrap().recv();
                match rx {
                    Ok(data) => {
                        println!("Worker{} received: {}", i, data); // вывожу сообщение и номер воркера
                    }
                    Err(_) => {
                        // Если канал закрыт, выходим из цикла
                        break;
                    }
                }
            }
        });
    }
    let _ = ctrl_c().await;

    for i in 0..n {
        let _ = i.await;
    }
    // Главный поток будет постоянно писать данные в канал
    let mut count = 0;
    loop {
        let data = format!("Message {}", count);
        tx.send(data).unwrap();
        count += 1;
        tokio::time::sleep(Duration::from_secs(1)).await; // Пауза на 1 секунду
    }
}