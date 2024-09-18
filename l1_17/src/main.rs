/*
Реализовать структуру-счетчик,которая будет инкрементироваться в конкурентной среде.
По завершению программа должна выводить итоговое значение счетчика.
 */

use std::sync::{Arc, Mutex};
use std::thread;

struct Counter {
    value: usize,
}
impl Counter {
    fn new() -> Self {
        Counter {
            value: 0
        }
    }

    fn increment(&mut self) {
        self.value += 1;
    }
    fn get(&self) -> usize {
        self.value
    }
}
fn main() {
    let counter = Arc::new(Mutex::new(Counter::new()));
    let mut handles = vec![];

    let num_threads = 4;
    let increment_per_thread = 1000;
    for _ in 0..num_threads {
        let counter_clone = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            for _ in 0..increment_per_thread {
                let mut counter_lock = counter_clone.lock().unwrap();
                counter_lock.increment();
            }
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }
    let result = counter.lock().unwrap();
    println!("Result: {}", result.get());
}
