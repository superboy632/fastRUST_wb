fn main() {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).expect("can't read line");
    let n:i32 = input.trim().parse().expect("Incorrect number");

    // обрабатываем введеное число на очень большое число и отрицательное, максимальное число 63, так как 64 бита
    if n > 63 || n < 0 {
        eprintln!("Недопустимый диапазон занчений");
        std::process::exit(1);
    }

    let mut input = String::new();
    std::io::stdin().read_line(&mut input).expect("can't read line");
    let number:i64 = input.trim().parse().expect("Incorrect number");

    // создаем мут переменную
    let mut a: i64 = number;

    // делаем битовый сдвиг на n, каждый сдвиг на 1 бит это умножение на 2 числа
    a = a << n;

    println!("{}", a);
}