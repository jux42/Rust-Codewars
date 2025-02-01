pub fn valid_isbn10(isbn: &str) -> bool {
    
    match isbn.len() == 10 && isbn[0..9].parse::<i32>().is_ok() {
        true => {
            let mut result = 0;
            isbn.chars().enumerate().for_each(|(index, c)| {
                let num = if c == 'X' { 10 } else { c as i32 - 0x30 };

                result += num * (index + 1) as i32;
            });
            result % 11 == 0
        }
        _ => false
    }
}
