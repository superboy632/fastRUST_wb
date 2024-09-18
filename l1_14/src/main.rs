use std::any::*;

fn type_variable<T: Any>(_: &T) {
    let type_id = TypeId::of::<T>();
    if type_id  == TypeId::of::<i32>() {
        println!("Тип переменной: i32");
    }
    if type_id  == TypeId::of::<i64>() {
        println!("Тип переменной: i64");
    }
    if type_id  == TypeId::of::<i128>() {
        println!("Тип переменной: i128");
    }
    if type_id  == TypeId::of::<f64>() {
        println!("Тип переменной: f64");
    }
    if type_id  == TypeId::of::<String>() {
        println!("Тип переменной: String");
    }
    if type_id  == TypeId::of::<&str>() {
        println!("Тип переменной: &str");
    }
    if type_id  == TypeId::of::<char>() {
        println!("Тип переменной: char");
    }
}

fn main() {
    let c:char = 'c';
    let number = 32;
    let big_number:i64 = 9223372036854775807;
    let string = "hello world";
    type_variable(&c);
    type_variable(&number);
    type_variable(&big_number);
    type_variable(&string);
}
