fn next_smaller_number(n: u64) -> Option<u64> {

    let mut n_str: Vec<char> = n.to_string().chars().collect();
    n_str.sort();
    let x = 10_usize.pow((n.to_string().len() - 1) as u32);
    
    for i in (x as u64..n).rev(){
        let mut i_chars :Vec<char> =  i.to_string().chars().collect();
        i_chars.sort();
        if i_chars == n_str {
            return  Some(i);
        }
    }
    None
}