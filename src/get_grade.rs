pub fn get_grade(s1: u16, s2: u16, s3: u16) -> char {

    let result = match (s1 + s2 + s3) / 3 {
        x if x >= 0 && x < 60 => 'F',
        x if x >=60 && x < 70 => 'D',
        x if x >=70 && x < 80 => 'C',
        x if x >=80 && x < 90 => 'B',
        x if x >=90 && x <= 100 => 'A',
        _ => 'Z'
    };
    result
}