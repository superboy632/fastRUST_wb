use std::collections::HashMap;

fn main() {
    let temperatures:[f64;8] = [-25.4, -27.0, 13.0, 19.0, 15.5, 24.5, -21.0, 32.5];
    let step:f64 = 10.0;

    // Хэш-таблица для хранения интервалов и значений
    let mut intervals: HashMap<(i32, i32), Vec<f64>> = HashMap::new();

    // Обрабатываем каждую температуру
    for &temp in &temperatures {
        // Находим интервал, в который попадает температура
        let lower_bound:i32 = ((temp / step).floor() * step) as i32; // Нижняя граница
        let upper_bound:i32 = lower_bound + step as i32; // Верхняя граница
        let interval:(i32, i32) = (lower_bound, upper_bound);

        // Добавляем температуру в соответствующий интервал
        intervals.entry(interval).or_insert(Vec::new()).push(temp);
    }

    // Выводим результаты
    for (interval, temps) in intervals {
        println!("[{:.1}, {:.1}): {:?}", interval.0, interval.1, temps);
    }
}
