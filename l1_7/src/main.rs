// use tokio::sync::mpsc;
// use tokio::time::{self, Duration};
//
// #[tokio::main]
// async fn main() {
//     let (tx, mut rx) = mpsc::channel(32);
//
//     // Запускаем задачу
//     let task = tokio::spawn(async move {
//         loop {
//             tokio::select! {
//                 _ = async {
//                     time::sleep(Duration::from_secs(1)).await;
//                     println!("Работа выполняется...");
//                 } => {},
//                 _ = rx.recv() => {
//                     println!("Получен сигнал остановки!");
//                     break;
//                 }
//             }
//         }
//     });
//
//     // Даем задаче немного поработать
//     time::sleep(Duration::from_secs(3)).await;
//
//     // Отправляем сигнал остановки
//     let _ = tx.send(()).await;
//
//     // Ждем завершения задачи
//     let _ = task.await.unwrap();
//     println!("Задача завершена.");
// }


use tokio_util::sync::CancellationToken;
use tokio::time::{self, Duration};

#[tokio::main]
async fn main() {
    let token = CancellationToken::new();
    let token_clone = token.clone();

    // Запускаем задачу
    let task = tokio::spawn(async move {
        loop {
            // Проверяем, был ли вызван `cancel`
            if token.is_cancelled() {
                println!("Задача отменена!");
                break;
            }
            time::sleep(Duration::from_secs(1)).await;
            println!("Работа выполняется...");
        }
    });

    // Даем задаче немного поработать
    time::sleep(Duration::from_secs(3)).await;

    // Отменяем задачу
    token_clone.cancel();

    // Ждем завершения задачи
    let _ = task.await.unwrap();
    println!("Задача завершена.");
}