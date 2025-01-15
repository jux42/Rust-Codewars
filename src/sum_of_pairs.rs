use std::collections::HashSet;

fn sum_pairs(ints: &[i8], s: i8) -> Option<(i8, i8)> {
    let mut check_list = HashSet::new();

    for &value in ints {
        let temp_result = s - value;
        if check_list.contains(&temp_result) {
            return Some((temp_result, value));
        }
        check_list.insert(value);
    }
    None
}
//     let mut result = None;
//     let mut ref_index = ints.len() - 1;
//     let mut iterator = 1;
//
//     for num in ints {
//         for i in iterator..ints.len() {
//             if num + ints[i] == s && i <= ref_index {
//                 ref_index = i;
//                 result = Some((*num, ints[i]));
//             }
//         }
//         iterator += 1;
//     }
//     result
// }
