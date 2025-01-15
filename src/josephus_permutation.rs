fn josephus<T: Clone + Copy>(xs: Vec<T>, k: usize) -> Vec<T> {
    let mut xs_clone = xs.clone();
    let mut xs_permuted = vec![];
    let mut iterator = k - 1;

    while !xs_clone.is_empty() {
        println!("{iterator}");
        if iterator >= xs_clone.len() {
            iterator %= xs_clone.len();
        }

        xs_permuted.push(xs_clone[iterator].clone());
        xs_clone.remove(iterator);

        iterator += (k - 1);
    }

    xs_permuted
}
