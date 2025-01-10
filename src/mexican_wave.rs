pub fn wave(s: &str) -> Vec<String> {
    let mut mex_vec = Vec::new();
    let mut s_chars: Vec<char> = s.to_string().chars().collect();
    println!("{:?}", s_chars);

    for iterator in 0..s_chars.len() {
        let mut s_string = String::new();
        for i in 0..s_chars.len() {
            match s_chars[i] {
                'a'..='z' => {
                    if i == iterator {
                        s_string.push_str(&s_chars[i].to_string().to_uppercase());
                    } else {
                        s_string.push(s_chars[i]);
                    }
                }
                ' ' => {
                    if i == iterator {
                        break;
                    } else {
                        s_string.push(s_chars[i]);
                    }
                }
                _ => continue,
            }

            if s_chars.len() == i + 1 {
                mex_vec.push(s_string.clone());
            }
        }
    }

    mex_vec
}
