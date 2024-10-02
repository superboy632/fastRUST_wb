```
fn as_chan(vs: &[i32]) -> std::sync::mpsc::Receiver<i32> {
    let (tx, rx) = std::sync::mpsc::channel();
    let handle = std::thread::spawn({
        let vs = vs.to_owned();
        move || {
            for v in vs {
                tx.send(v).unwrap(); std::thread::sleep(std::time::Duration::from_secs(1))
            }
            drop(tx);
        }
    });
    handle.join().unwrap();
    rx
}

fn merge( a: std::sync::mpsc::Receiver<i32>, b: std::sync::mpsc::Receiver<i32>) -> std::sync::mpsc::Receiver<i32> {

    let (tx, rx) = std::sync::mpsc::channel();

    let mut a_done = false;

    let mut b_done = false;

    loop {
        match a.try_recv() {
            Ok(i) => {
                tx.send(i).unwrap();
            }
            Err(_) => {
                a_done = true;
            }
        }
        match b.try_recv() {

            Ok(i) => {

                tx.send(i).unwrap();

            }

            Err(_) => {

                b_done = true;

            }
        }
        if a_done && b_done {

            break;
        }
    }
    rx
}

fn main() {
    let a = as_chan(&vec![1, 3, 5, 7]);

    let b = as_chan(&vec![2, 4, 6, 8]);

    let c = merge(a, b);

    for v in c.iter() {
        println!("{v:?}");
    }
}
```
В приведенном коде показана работа трех каналов. Создаются 2 канала a и b, потоки которых отправляют векторы цифр в канал. Так же вызывается третий канал, который собирает эти данные с двух каналов. Собирать данные он будет пока они поступают, когда оба канала перестанут передавать данные третий6 канал закроется.<br>
Вывод программы: 1 2 3 4 5 6 7 8