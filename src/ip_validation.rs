pub fn is_valid_ip(ip: &str) -> bool {
    let mut ip_vec: Vec<u8> = vec![];

    for n in ip.split('.') {
        if n.starts_with('0') && n != "0" {
            return false;
        }
        match n.parse::<u8>() {
            Ok(num) => ip_vec.push(num),
            Err(_) => return false,
        }
    }
    ip_vec.len()==4
}
