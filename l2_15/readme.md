```
fn main() {

let s1 = "hello";

let s2 = String::from("hello");

let s3 = s2.as_str();

let size_of_s1 = std::mem::size_of_val(s1);

let size_of_s2 = std::mem::size_of_val(&s2);

let size_of_s3 = std::mem::size_of_val(&s3);

println!("{:?}", size_of_s1);

println!("{:?}", size_of_s2);

println!("{:?}", size_of_s3);

}
```
- Создаются 3 переменные: 1) s1- строковый срез 2) s2 - String, память на который выделяется в куче 3) s3 - строковый срез
```
  let size_of_s1 = std::mem::size_of_val(s1);

  let size_of_s2 = std::mem::size_of_val(&s2);

  let size_of_s3 = std::mem::size_of_val(&s3);
```
- В данном блоке кода мы узнаем размер в байтах каждой переменной
1) s1 - 5 байт - каждый символ 1 байт
2) s2 - 24 байт, так как 64 битная система
3) s3 - 16 байт, точно не понимаю почему отличается от s1, но предполагаю из-за s2.as_str(), так как s3 указатель и он 16 байт в 64 битной системе

