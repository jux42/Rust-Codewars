fn rgb(r: i32, g: i32, b: i32) -> String {
    let convert = |value: i32| -> char {
        match value {
            0..=9 => (value as u8 + b'0') as char,
            10 => 'A',
            11 => 'B',
            12 => 'C',
            13 => 'D',
            14 => 'E',
            15 => 'F',
            _ => panic!("sthg went wrong with {}", value),
        }
    };

    let decompose_to_hex = |mut value: i32| -> (char, char) {
        match value {
            x if x < 0 => value = 0,
            x if x > 255 => value = 255,
            _ => {}
        }
        let first = value / 16;
        let second = value % 16;
        
        (convert(first), convert(second))
    };
    let (r1, r2) = decompose_to_hex(r);
    let (g1, g2) = decompose_to_hex(g);
    let (b1, b2) = decompose_to_hex(b);

    format!("{r1}{r2}{g1}{g2}{b1}{b2}")
}
