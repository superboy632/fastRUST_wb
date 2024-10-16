use std::{thread, time};
use std::sync::mpsc;


fn main() {
    let n:i32 = 10;
    let mut v:   Vec<i32> = Vec::new(); // создаем вектор
    let mut handles = vec![]; // вектор для ручек с потоков
    for i in 0..n {
        v.push(i); // заполняем вектор значениями
    }
    let (tx1, rx1) = mpsc::channel(); // создаем первый канал
    let (tx2, rx2) = mpsc::channel(); // создаем второй канал
    // создаем поток для отправки данных в первый канал
    let child = thread::spawn(move || {
        for k  in 0..n {
            tx1.send(v[k as usize]).unwrap(); // отправляем число в канал
            thread::sleep(time::Duration::from_secs(2)); // сделаем задержку отправления 2 секунды
        }
        drop(tx1);
    });
    handles.push(child);

    let child1 = thread::spawn(move || {
        for k in 0..n {
            let buffer= rx1.recv().unwrap(); // принимаем элементы массива из первого канала
            tx2.send(buffer*buffer).unwrap(); // отправляем во второй канал квадраты массива
        }
        drop(tx2);
    });
    handles.push(child1);

    let child2 = thread::spawn(move || {
        for k in 0..n {
            let buffer;
            buffer = rx2.recv().unwrap();
            println!("Квадрат {} элемента массива: {:?}", k, buffer); // выводим квадраты массива
        }
    });
    handles.push(child2);

    for h in handles {
        h.join().unwrap();
    }
}
