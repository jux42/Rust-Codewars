pub fn longest_consec(strarr: Vec<&str>, k: usize) -> String {

    if k == 0 || strarr.is_empty() || k>strarr.len () {
        return "".to_string()
    }
    let mut consec = String::new();
    let mut longest = String::new();
    for i in 0..=strarr.len()-k {
        consec = strarr[i..(i+k)].join("");
        if consec.len() > longest.len() {
            longest = consec;
        }
    }
    longest
}