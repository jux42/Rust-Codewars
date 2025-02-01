pub fn beeramid(bonus: i32, price: f32) -> usize {
    let quantity = (bonus.clamp(0, i32::MAX) as f32 / price).floor() as i32;
    let level_calc = |q| -> usize {
        let mut level = 1;
        let mut q_left = q;

        loop {
            match level * level {
                _l if _l <= q_left => q_left -= level * level,
                _l if _l > q_left => {
                    level -= 1;
                    break;
                }
                _ => {}
            };
            level += 1;
        }
        level as usize
    };

    level_calc(quantity)
}
#[cfg(test)]
mod tests {
    use super::beeramid;

    #[test]
    fn sample_tests() {
        assert_eq!(beeramid(9, 2.0), 1);
        assert_eq!(beeramid(10, 2.0), 2);
        assert_eq!(beeramid(11, 2.0), 2);
        assert_eq!(beeramid(21, 1.5), 3);
        assert_eq!(beeramid(454, 5.0), 5);
        assert_eq!(beeramid(455, 5.0), 6);
        assert_eq!(beeramid(4, 4.0), 1);
        assert_eq!(beeramid(3, 4.0), 0);
        assert_eq!(beeramid(0, 4.0), 0);
        assert_eq!(beeramid(-1, 4.0), 0);
        assert_eq!(beeramid(10000, 4.0), 19);
    }
}
