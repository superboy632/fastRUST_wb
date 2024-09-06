pub trait Action {
    fn say(&self);
}

pub struct Person {
    pub NAME: String
}

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
