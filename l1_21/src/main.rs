fn main() {
    let a: f64 = 2.0_f64.powf(21.0); // Пример значения больше 2^20
    let b: f64 = 3.0_f64.powf(21.0); // Пример значения больше 2^20

    // Проверка на выполнение условие (> 2^20)
    if a > 2.0_f64.powf(20.0) && b > 2.0_f64.powf(20.0) {
        println!("a: {}", a);
        println!("b: {}", b);

        // Сложение
        let sum = a + b;
        println!("Сложение: a + b = {}", sum);

        // Вычитание
        let difference = a - b;
        println!("Вычитание: a - b = {}", difference);

        // Умножение
        let product = a * b;
        println!("Умножение: a * b = {}", product);

        // Деление
        if b != 0.0 {
            let quotient = a / b;
            println!("Деление: a / b = {}", quotient);
        } else {
            println!("Ошибка: Деление на ноль!");
        }
    } else {
        println!("Числа должны быть больше 2^20.");
    }
}
