// Реализовать паттерн «адаптер» на любом примере.
// Интерфейс, который мы хотим, чтобы наш адаптер реализовывал
trait Rectangle {
    fn area(&self) -> f64;
}

// Структура, представляющая прямоугольник
struct BasicRectangle {
    width: f64,
    height: f64,
}

impl Rectangle for BasicRectangle {
    fn area(&self) -> f64 {
        self.width * self.height
    }
}

// Структура, представляющая круг
struct Circle {
    radius: f64,
}

// Создаем адаптер, чтобы круг можно было использовать как прямоугольник
struct CircleAdapter<'a> {
    circle: &'a Circle,
}

impl<'a> Rectangle for CircleAdapter<'a> {
    fn area(&self) -> f64 {
        // Площадь круга = π * r^2
        std::f64::consts::PI * self.circle.radius * self.circle.radius
    }
}

fn main() {
    let rectangle = BasicRectangle { width: 10.0, height: 5.0 };
    let circle = Circle { radius: 3.0 };

    let circle_adapter = CircleAdapter { circle: &circle };

    println!("Площадь прямоугольника: {}", rectangle.area());
    println!("Площадь круга: {}", circle_adapter.area());
}
