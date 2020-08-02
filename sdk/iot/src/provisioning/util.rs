use heapless::consts::U20;
use heapless::{String, Vec};

pub fn u64_to_string(value: u64) -> String<U20> {
    const ZERO: u8 = b'0';
    let mut res: String<U20> = String::new();
    if value == 0 {
        res.push('0').unwrap();
        res.truncate(1);
        return res;
    }
    let mut digits: Vec<char, U20> = Vec::new();
    let mut div = value;
    while div > 0 {
        let rem: u8 = (div % 10) as u8;
        digits.push((ZERO + rem as u8) as char).unwrap();
        //div -= rem;
        div /= 10;
    }
    digits.reverse();
    for digit in digits {
        res.push(digit).unwrap();
    }
    res.truncate(res.len());
    res
}

#[cfg(test)]
mod tests_u64_to_string {
    use super::*;
    #[test]
    fn u64_max_decodes() {
        let max = u64_to_string(u64::MAX);
        assert_eq!("18446744073709551615", max);
    }
    #[test]
    fn u64_min_decodes() {
        let min = u64_to_string(u64::MIN);
        assert_eq!("0", min);
    }
}
