pub fn retry_calc_delay(attempt: i16, retry_delay_msec: i32, max_retry_delay_msec: i32) -> i32 {
    let exponential_retry_after = retry_delay_msec
        * (if attempt <= 30 {
            1 << attempt
        } else {
            i32::MAX
        }); // scale exponentially

    if exponential_retry_after > max_retry_delay_msec {
        max_retry_delay_msec
    } else {
        exponential_retry_after
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn after_31_attempts_retry_is_max_retry_delay_is_used() {
        assert_eq!(retry_calc_delay(31, 1, 0), 0);
        assert_eq!(retry_calc_delay(31, 1, 1), 1);
        assert_eq!(retry_calc_delay(31, 1, 2), 2);
    }
}
