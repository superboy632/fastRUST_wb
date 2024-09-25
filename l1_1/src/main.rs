// создаем трейт ACTION, грубо говоря интерфейс
pub trait Action {
    fn say(&self);
}
// создаем структуру Person
pub struct Person {
    pub name: String
}
/* реализуем типаж, после impl ставим имя трейта, который хотим реализовать и после ключевого слова for
 указываем имя типа, для которого хотим реализовать
*/
impl Action for Person {
    fn say(&self) {
        println!("Hello, {}", self.name);
    }
}


fn main() {
    let person = Person {
        name: String::from("Grigory"),
    };

    person.say();
}
