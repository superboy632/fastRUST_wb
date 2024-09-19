fn main() {
    let mut vec = vec![1, 2, 3, 4, 5];

    let i = 4; // Индекс элемента, который нужно удалить
    if i < vec.len() {
        println!("Исходный вектор: {:?}", vec);
        let removed_element = vec.remove(i);
        println!("Удаленный элемент: {}", removed_element);
        println!("Получившийся вектор: {:?}", vec);
    } else {
        println!("Индекс вне диапазона");
    }
}