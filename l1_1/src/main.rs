// создаем трейт ACTION, грубо говоря интерфейс
pub trait Action {
    fn say(&self);
}
// создаем структуру Person
pub struct Person {
    pub NAME: String
}
/* реализуем типаж, после impl ставим имя трейта, который хотим реализовать и после ключевого слова for
 указываем имя типа, для которого хотим реализовать
*/
impl Action for Person {
    fn say(&self) {
        println!("Hello, {}", self.NAME);
    }
}


fn main() {
    let person = Person {
        NAME: String::from("Grigory"),
    };

    person.say();
}
