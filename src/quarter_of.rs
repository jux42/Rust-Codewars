pub(crate) fn quarter_of(month: u8) -> u8 {
    let mut quarter = 0;
    for i in 1..=month {
        if month == i {
            if i % 3 == 0 {
                quarter = i / 3;
            }
            if i % 3 != 0 {
                quarter = (i / 3) + 1;
            }
        }
    }
    quarter
}
