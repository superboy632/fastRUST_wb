use flume::{unbounded, Receiver};
use tokio::time::{self, Duration};
use tokio::signal;


async fn workers(number: usize, rx: Receiver<String>) {
    loop {
        tokio::select! {
            message = rx.recv_async() => {
                match message {
                    Ok(data) => {
                    println!("Worker{} received: {}", number, data); // вывожу сообщение и номер воркера
                    },
                    Err(_) => {
                        // Если канал закрыт, выходим из цикла
                        break;
                    },
                }
            },
            _ = signal::ctrl_c() => {
                println!("Воркер {} завершает работу", number);
                break;
            },
        }
    }
}

async fn task(num_workers: usize) {
    let (tx, rx) = unbounded();

    for i in 0..num_workers {
        let rx1 = rx.clone();
        tokio::spawn(workers(i, rx1));
    }

    let mut count = 0;
    loop {
        let data = format!("Message {}", count);
        tx.send(data).unwrap();
        count += 1;
        tokio::time::sleep(Duration::from_secs(1)).await; // Пауза на 1 секунду
    }

}

#[tokio::main]
async fn main() {
    let mut num_workers = String::new();
    std::io::stdin().read_line(&mut num_workers).expect("can't read line");
    let n = num_workers.trim().parse().expect("Incorrect number");


    tokio::spawn(async move {
        task(n).await;
    });

    // Ждем сигнал Ctrl+C
    tokio::signal::ctrl_c().await.expect("Error while waiting for Ctrl+C");

    time::sleep(Duration::from_secs(1)).await;

}
