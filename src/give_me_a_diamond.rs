pub fn print(n: i32) -> Option<String> {
    let mut output = "".to_string();
    let mut x = 1;
    let star = "*";
    let space = " ";

    match n {
        n if n > 1 && n % 2 != 0 => {
            while x <= n {
                let freq = (n - x) / 2;
                output.push_str(&space.repeat(freq as usize));
                output.push_str(&star.repeat(x as usize));
                output.push_str("\n");
                if x == n {
                    break;
                } else {
                    x += 2
                };
            }
            while x > 1 {
                x -= 2;
                let freq = (n - x) / 2;
                output.push_str(&space.repeat(freq as usize));
                output.push_str(&star.repeat(x as usize));
                output.push_str("\n");
            }
            Some(output)
        }

        n if n == 1 => {
            Some("*\n".to_string())
        }

        _ => None
    }
}
