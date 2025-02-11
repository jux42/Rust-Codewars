use std::collections::HashMap;

pub fn row_sum_odd_numbers(n:i64) -> i64 {

    let mut odd_triangle : HashMap<i64, Vec<i64>> = HashMap::new();
    let mut last_odd = 1;

    for i in (1..=n) {
        let mut odd_list = Vec::new();
        for j in 1..=i {
            last_odd+= if i>1 {2} else {0}; ;
            odd_list.push(last_odd);
        }
        odd_triangle.insert(i, odd_list);
    }
    let odd_array = odd_triangle.get(&n).unwrap();
    odd_array.iter().sum()
}