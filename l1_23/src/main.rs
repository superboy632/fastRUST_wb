use std::f64;

#[derive(Debug)]
struct Point {
    x: f64,
    y: f64,
}

impl Point {
    // Конструктор для создания новой точки
    fn new(x: f64, y: f64) -> Self {
        Point { x, y }
    }

    // Метод для вычисления расстояния до другой точки
    fn distance(&self, other: &Point) -> f64 {
        ((self.x - other.x).powi(2) + (self.y - other.y).powi(2)).sqrt()
    }
}

fn main() {
    // Создаем две точки
    let point1 = Point::new(1.0, 2.0);
    let point2 = Point::new(4.0, 6.0);

    // Вычисляем расстояние между ними
    let distance = point1.distance(&point2);

    println!("Расстояние между точками: {:.2}", distance);
}
