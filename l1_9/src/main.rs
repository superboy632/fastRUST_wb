fn main() {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).expect("can't read line");
    let n:i32 = input.trim().parse().expect("Incorrect number");

    // обрабатываем введеное число на очень большое число и отрицательное
    if n > 63 || n < 0 {
        eprintln!("Недопустимый диапазон занчений");
        std::process::exit(1);
    }

    let mut a: i64 = 1;

    a = a << n;

    println!("{}", a);
}