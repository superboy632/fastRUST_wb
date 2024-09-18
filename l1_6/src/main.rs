//Разработать программу, которая будет последовательно отправлять значения в канал, а с другой стороны канала — читать.
// По истечению N секунд программа должна завершаться
use std::sync::mpsc;
use std::time::Duration;
use tokio::time;

/*
функция stream - процесс, который создает 2 таска, один sender другой receiver. Канал закрывается как только пройдет 4 секунды
 */
async fn stream(sec: Duration) {
    let (tx,rx) = mpsc::channel();
    let sender = {
        let tx1 = tx.clone();
        tokio::spawn( async move{
            let mut count = 0;
            while count < 7 {
                if let Err(_) = tx1.send(count) {
                    eprintln!("Закрытие канала");
                    return
                }
                count += 1;
                time::sleep(Duration::from_secs(1)).await;
            }
        })
    };

    let receiver = {
        tokio::spawn(async move{
            let end_time = time::Instant::now() + sec;
            while time::Instant::now() < end_time {
                if let Ok(value) = rx.recv() {
                    println!("Received: {}", value);
                } else {
                    break; // Канал закрыт
                }
            }
        })
    };
    let _ = tokio::try_join!(sender, receiver); // ожидаем закрытие потоков
}

#[tokio::main]
async fn main() {
    stream(Duration::from_secs(4)).await;
}