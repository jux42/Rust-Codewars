
pub(crate) fn xo(string: &'static str) -> bool {


    let mut counter_x = string
        .to_lowercase()
        .matches('x')
        .count();
    let  counter_o = string
        .to_lowercase()
        .matches('o')
        .count();

        counter_x == counter_o

}