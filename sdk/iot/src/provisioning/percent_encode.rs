use azure_sdk_for_rust_common::error::AZ_ERROR_INSUFFICIENT_SPAN_SIZE;
use heapless::consts::U128;
use heapless::String;

pub fn encode(value: &str) -> Result<String<U128>, &'static str> {
    let mut result: String<U128> = String::new();
    if value.is_empty() {
        // return 0 len buffer
        result.truncate(0);
        return Ok(result);
    }
    let extra_space_have = (128 - value.len()) / 2;
    if extra_space_have >= value.len() {
        // We know that there's enough space even if every character gets encoded.
        for c in value.chars() {
            push_char_with_needed_encoding(&mut result, c).unwrap();
        }
    } else {
        // We may or may not have enough space, given whether the input needs much encoding or not.
        for c in value.chars() {
            if push_char_with_needed_encoding(&mut result, c).is_err() {
                return Err(AZ_ERROR_INSUFFICIENT_SPAN_SIZE);
            }
        }
    }
    result.truncate(result.len());
    Ok(result)
}
fn push_char_with_needed_encoding(result: &mut String<U128>, c: char) -> Result<(), ()> {
    if should_encode(c) {
        result.push('%')?;
        let upper = (c as u8) >> 4;
        result.push(number_to_upper_hex(upper))?;
        let lower = (c as u8) & 0x0F;
        result.push(number_to_upper_hex(lower))?;
    } else {
        result.push(c)?;
    }
    Ok(())
}
fn should_encode(c: char) -> bool {
    match c {
        '-' | '_' | '.' | '~' => false,
        _ => !(('0' <= c && c <= '9') || ('A' <= c && c <= 'Z') || ('a' <= c && c <= 'z')),
    }
}

fn number_to_upper_hex(number: u8) -> char {
    let hex_upper_offset: u8 = (b'A') - 10;
    let zero = b'0';
    let result = number + (if number < 10 { zero } else { hex_upper_offset });
    result as char
}

#[cfg(test)]
mod tests_encode {
    use super::*;
    #[test]
    fn truth() {
        let result = encode("http://www.example.com/#anchor").unwrap();
        assert_eq!("http%3A%2F%2Fwww.example.com%2F%23anchor", result.as_str());
    }
}

#[cfg(test)]
mod tests_should_encode {
    use super::*;
    #[test]
    fn truth() {
        let literals = ['-', '_', '.', '~'];
        for item in &literals {
            assert!(!should_encode(*item))
        }
    }

    #[test]
    fn number_should_not_encode() {
        let literals = ['0', '1', '2', '3', '4', '5', '6', '7', '8', '9'];
        for item in &literals {
            assert!(!should_encode(*item))
        }
    }

    #[test]
    fn alphabet_should_not_encode() {
        let lower_a = b'a';
        for i in 0..26 {
            let current = lower_a + i;
            assert!(!should_encode(current as char));
        }
        let upper_a = b'A';
        for i in 0..26 {
            let current = upper_a + i;
            assert!(!should_encode(current as char));
        }
    }

    #[test]
    fn the_rest_should_encode() {
        let lower: core::ops::RangeInclusive<u8> = 0..=64;
        let middle: core::ops::RangeInclusive<u8> = 91..=96;
        let upper: core::ops::RangeInclusive<u8> = 123..=127;
        for item in lower {
            if item == 45 {
                continue; // -
            }
            if item == 46 {
                continue; // .
            }
            if item >= 48 || item <= 57 {
                continue; //numbers
            }
            assert!(should_encode(item as char));
        }
        for item in middle {
            if item == 95 {
                continue;
            } // _
            assert!(should_encode(item as char));
        }
        for item in upper {
            if item == 126 {
                continue;
            } // ~
            assert!(should_encode(item as char));
        }
    }
}

#[cfg(test)]
mod tests_number_to_upper_hex {
    use super::*;
    #[test]
    fn zero_through_nine_encode_normally() {
        let numbers = 0..=9;
        let chars = '0'..='9';
        let pairs = numbers.zip(chars);
        for (n, c) in pairs {
            assert_eq!(number_to_upper_hex(n), c);
        }
    }
    #[test]
    fn ten_through_fifteen_encode_as_hex_upper() {
        let numbers = 10..=15;
        let chars = 'A'..='F';
        let pairs = numbers.zip(chars);
        for (n, c) in pairs {
            assert_eq!(number_to_upper_hex(n), c);
        }
    }
}
