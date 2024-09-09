use std::sync::{mpsc, Arc, Mutex};
use std::time::Duration;
use tokio::signal;
use tokio::signal::ctrl_c;

#[tokio::main]
async fn main() {
    let (tx, rx) = mpsc::channel();
    let rx = Arc::new(Mutex::new(rx)); //

    let mut num_workers = String::new();
    std::io::stdin().read_line(&mut num_workers).expect("can't read line");
    let n = num_workers.trim().parse().expect("Incorrect number");

    if  n < 0 {
        eprintln!("Недопустимый диапазон занчений");
        std::process::exit(1);
    }
    let mut workers = vec![];
    // Запускаем воркеры
    for i in 0..n {
        let rx1 = Arc::clone(&rx);
        let handle = tokio::spawn(async move {
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
        workers.push(handle);
    }
    let ctrl_c_handle = tokio::spawn(async {
        signal::ctrl_c().await.unwrap();
        println!("ctrl_c detected");
    });
    // Главный поток будет постоянно писать данные в канал
    let mut count = 0;
    loop {
        // tokio::select! {
        //     _ = ctrl_c_handle => {
        //         println!("Ctrl+c received");
        //         break;
        //     }
        //     _ = tokio::time::sleep(Duration::from_secs(1)) => {
        //         let data = format!("Message {}", count);
        //         tx.send(data).unwrap();
        //         count += 1;
        //     }
        }
}
