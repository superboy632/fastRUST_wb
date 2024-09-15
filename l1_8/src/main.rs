/*
HashMap  — это структура данных, которая представляет собой ассоциативный массив.
Она хранит пары «ключ-значение», позволяя эффективно хранить и извлекать данные по ключу.
Ниже приведен пример работы данной структуры данных в многопоточном режиме. Чтобы избежать блокировки данных приходится использовать
Arc и Mutex
 */

// use std::collections::HashMap;
// use std::sync::{Arc, Mutex};
// use std::thread;

// fn main() {
//     let map = Arc::new(Mutex::new(HashMap::new()));
//     let mut handles = vec![];

//     for i in 0..10 {
//         let map_clone = Arc::clone(&map);
//         let handle = thread::spawn(move || {
//             map_clone.lock()
//             .unwrap()
//             .insert(i, i * 10);
//             println!("Inserted: {} -> {}", i, i * 10);
//         });
//         handles.push(handle);
//     }

//     for handle in handles {
//         handle.join().unwrap();
//     }

//     let map = map.lock().unwrap();
//     println!("Final map: {:?}", *map);
// }

/*
Данная программы показывает работу DashMap, DashMap - реализация паралельного ассоциативного массива/хэшмапы.
Она оптимизирована для использования в многопоточном режиме и позволяет безопасно обращаться к данным из разных потоков, избегая блокировок
которые могут привести к снижению производительности
 */
use dashmap::DashMap;
use std::thread;

fn main() {
    let map = DashMap::new(); // создаем DashMap
    let mut handles = vec![];

    for i in 0..10 {
        let map_clone = map.clone();
        let handle = thread::spawn(move || {
            map_clone.insert(i, i * 10);
            println!("Inserted: {} -> {}", i, i * 10);
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    // Печатаем содержимое DashMap
    for r in map.iter() {
        println!("Key: {}, Value: {}", r.key(), r.value());
    }
}