fn main() {
    let line = String::from("a4b2c3d");
    let result = unpacking(line);
    println!("{result}");
}

fn unpacking(line: String) -> String {
    let mut result = String::new();
    /* chars() возвращает итератор по символам строки
    peekable повзоляет смотреть на значения, не доходя до них итератором
    */
    let mut chars = line.chars().peekable();
    while let Some(current_char) = chars.next() {
        if current_char.is_digit(10) {
            return result
        }
        if current_char.is_alphabetic() {
            result.push(current_char);
        }
         if let Some(_) = chars.peek() {
            let mut count = String::new();
             while let Some(&next_digit) = chars.peek() {
                 if next_digit.is_digit(10) {
                     count.push(next_digit);
                     chars.next();
                 } else {
                     break;
                 }
             }
             if let Ok(number) = count.parse::<i32>() {
                 for _ in 0..number - 1 {
                     result.push(current_char);
                 }
             }
         }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_standard() {
        let line = String::from("a4b2c3d");
        let result = "aaaabbcccd";
        assert_eq!(result, unpacking(line));
    }

    #[test]
    fn case_wrong() {
        let line = String::from("45");
        let result = "";
        assert_eq!(result, unpacking(line));
    }

    #[test]
    fn case_none_numbers() {
        let line = String::from("abcd");
        let result = "abcd";
        assert_eq!(result, unpacking(line));
    }

    #[test]
    fn case_empty() {
        let line = String::from("");
        let result = "";
        assert_eq!(result, unpacking(line));
    }

    #[test]
    fn case_wow() {
        let line = String::from("a4b2c3d333");
        let result = "aaaabbcccddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddd";
        assert_eq!(result, unpacking(line));
    }
}
