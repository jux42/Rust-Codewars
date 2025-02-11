pub fn good_vs_evil(good: &str, evil: &str) -> String {

    let good_worths = [1, 2, 3, 3, 4, 10];
    let evil_worths = [1, 2, 2, 2, 3, 5, 10];

    let mut good_count: Vec<i32> = good
        .split_whitespace()
        .map(|s| s.parse::<i32>().unwrap())
        .zip(good_worths.iter())
        .map(|(good, good_worths)| good * good_worths)
        .collect();

    let mut evil_count: Vec<i32> = evil
        .split_whitespace()
        .map(|s| s.parse::<i32>().unwrap())
        .zip(evil_worths.iter())
        .map(|(evil, evil_worths)| evil * evil_worths)
        .collect();

    let good_sum: i32 = good_count.iter().sum();
    let evil_sum: i32 = evil_count.iter().sum();

    if good_sum > evil_sum {
        return "Battle Result: Good triumphs over Evil".to_string();
    } else if good_sum < evil_sum {
        return "Battle Result: Evil eradicates all trace of Good".to_string();
    } else if good_sum == evil_sum {
        return "Battle Result: No victor on this battle field".to_string();
    }

    "sthg went wrong".to_string()
}
