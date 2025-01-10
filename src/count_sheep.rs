fn count_sheep(n: u32) -> String {
    let mut sheep_count = String::from("");
    let mut i = 1;
    while i <= n {
        sheep_count.push_str(format!("{} sheep...", i.to_string()).as_str());
        i += 1;
    }
    sheep_count
}
