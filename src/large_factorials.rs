
//Todo : deal with very large numbers (>u128)
pub fn factorial(x: u32) -> String {

    let mut y: u128 = 1;
    
    for n in 1..=x as u128{
        
        y*=n;
        
    }
    y.to_string()
}
