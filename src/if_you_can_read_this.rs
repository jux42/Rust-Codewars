pub fn to_nato(words: &str) -> String {
    let mut translation = String::new();

    for letter in words.to_uppercase().chars().collect::<Vec<_>>() {
        match letter {
            ' ' => continue,
            ',' | '.' | '!' | '?' => {
                translation.push(letter);
                translation.push(' ');
            },
            'A'..='Z' => {
                translation.push_str(NATO[&letter]);
                translation.push(' ');
            }
            _ => return "sthg went wrong".to_string(),
        }
    }
    println!("{translation}");
    translation.trim().to_string()
}

use once_cell::sync::Lazy;
use std::collections::HashMap;

#[rustfmt::skip]
pub static NATO: Lazy<HashMap<char, &'static str>> = Lazy::new(|| {
    [
        ('A', "Alfa"), ('B', "Bravo"), ('C', "Charlie"), ('D', "Delta"),
        ('E', "Echo"), ('F', "Foxtrot"), ('G', "Golf"), ('H', "Hotel"),
        ('I', "India"), ('J', "Juliett"), ('K', "Kilo"), ('L', "Lima"),
        ('M', "Mike"), ('N', "November"), ('O', "Oscar"), ('P', "Papa"),
        ('Q', "Quebec"), ('R', "Romeo"), ('S', "Sierra"), ('T', "Tango"),
        ('U', "Uniform"), ('V', "Victor"), ('W', "Whiskey"), ('X', "Xray"),
        ('Y', "Yankee"), ('Z', "Zulu"),
    ]
    .iter()
    .copied()
    .collect()
});
