//Реализовать бинарный поиск встроенными методами языка.
/*
суть данного метода заключается в делении массива на части и сравнение с искомым значением
поиск эффективен с большими массивами, сложность - O(log(n))
недостатоком данного метода является то, что массив должен быть отсортирован
 */
fn fast_search<T: Ord>(v: &mut [T], q: T) -> Option<usize> {
    let mut min_index = 0;
    let mut max_index = v.len() - 1;
    while max_index >= min_index {
        let mid_index = (min_index + max_index) / 2;
        if v[mid_index] == q  {
            return Some(mid_index);
        } else if v[mid_index] < q {
            min_index = mid_index + 1;
        } else {
            max_index = mid_index - 1;
        }
    }
    None
}


fn main() {
    let mut v = [1, 3, 7, 8, 9, 12, 15];
    let q = 9;
    match fast_search(&mut v, q) {
        Some(i) => println!("{}", v[i]),
        None => println!("значение не найдено"),
    }
}
