use base64::{decode_config_slice, encode_config_slice};

use heapless::String;
use heapless::Vec;

use azure_sdk_for_rust_common::error::AZ_ERROR_INSUFFICIENT_SPAN_SIZE;
pub const AZ_ERROR_UNABLE_TO_DECODE_BASE64: &str =
    "The given input could not be converted to base64.";

pub fn base64_encode<B, T: AsRef<[u8]>>(input: T) -> Result<String<B>, &'static str>
where
    B: heapless::ArrayLength<u8>,
{
    let input_bytes = input.as_ref();
    let safe_buffer_size = input_bytes.len() * 4 / 3 + 4;
    let buffer_size = B::to_usize();

    let mut buffer: heapless::Vec<u8, B> = heapless::Vec::new();
    // Default length is 0, make the vector 'filled' for the slice encoding
    if safe_buffer_size > buffer_size {
        // we may get a panic, use the max buffer we can.
        if buffer.resize_default(buffer_size).is_err() {
            return Err(AZ_ERROR_INSUFFICIENT_SPAN_SIZE);
        }
    } else {
        // our main buffer is large enough to safely hold the encoded data.
        // resize to the minimum we need.
        let preferred_buffer_size = core::cmp::min(safe_buffer_size, buffer_size);
        if buffer.resize_default(preferred_buffer_size).is_err() {
            return Err(AZ_ERROR_INSUFFICIENT_SPAN_SIZE);
        }
    }

    let bytes_written = encode_config_slice(&input, base64::STANDARD, &mut buffer);
    buffer.truncate(bytes_written);
    let encoded_result: Result<String<B>, core::str::Utf8Error> = String::from_utf8(buffer);

    if let Ok(encoded_value) = encoded_result {
        return Ok(encoded_value);
    }
    Err(AZ_ERROR_INSUFFICIENT_SPAN_SIZE)
}

pub fn base64_decode<B, T: AsRef<[u8]>>(input: T) -> Result<Vec<u8, B>, &'static str>
where
    B: heapless::ArrayLength<u8>,
{
    let input_bytes = input.as_ref();
    let safe_buffer_size = (input_bytes.len() + 3) / 4 * 3;
    let buffer_size = B::to_usize();

    if input_bytes.len() > buffer_size {
        return Err(AZ_ERROR_INSUFFICIENT_SPAN_SIZE);
    }

    let mut buffer: Vec<u8, B> = heapless::Vec::new();
    // Default length is 0, make the vector 'filled' for the slice encoding
    if safe_buffer_size > buffer_size {
        // we may get a panic, use the max buffer we can.
        if buffer.resize_default(buffer_size).is_err() {
            return Err(AZ_ERROR_INSUFFICIENT_SPAN_SIZE);
        }
    } else {
        // our main buffer is large enough to safely hold the encoded data.
        // resize to the minimum we need.
        let preferred_buffer_size = core::cmp::min(safe_buffer_size, buffer_size);
        if buffer.resize_default(preferred_buffer_size).is_err() {
            return Err(AZ_ERROR_INSUFFICIENT_SPAN_SIZE);
        }
    }

    match decode_config_slice(&input, base64::STANDARD, &mut buffer) {
        Ok(bytes_written) => buffer.truncate(bytes_written),
        Err(_) => return Err(AZ_ERROR_UNABLE_TO_DECODE_BASE64),
    }
    Ok(buffer)
}

#[cfg(test)]
mod base64_encoding_decoding_tests {
    use super::*;
    use heapless::consts::{U128, U256, U4, U64};
    #[test]
    fn string_slice_encoded_data_can_be_base64_encoded_to_heapless_strings() {
        let expected = "YWFhYWFhYWE=";
        let data = "aaaaaaaa";

        let result = base64_encode::<U256, _>(&data).unwrap();
        assert_eq!(result.as_str(), expected);
    }
    #[test]
    fn heapless_string_encoded_data_can_be_base64_encoded_to_heapless_strings() {
        let expected = "YWFhYWFhYWE=";
        let mut data: String<U128> = String::new();
        data.push_str("aaaaaaaa").unwrap();

        let result = base64_encode::<U128, _>(&data).unwrap();
        assert_eq!(result.as_str(), expected);
    }
    #[test]
    fn byte_encoded_data_can_be_base64_encoded_to_heapless_strings() {
        let expected = "YWFhYWFhYWE=";
        let data: [u8; 8] = [b'a'; 8];

        let result = base64_encode::<U64, _>(&data).unwrap();
        assert_eq!(result.as_str(), expected);
    }
    #[test]
    #[should_panic]
    fn when_the_buffer_is_to_small_to_hold_the_encoded_data_the_base64_lib_will_panic() {
        let data: [u8; 8] = [b'a'; 8];

        let _ = base64_encode::<U4, _>(&data).unwrap();
    }
    #[test]
    fn heapless_vector_encoded_data_can_be_base64_encoded_to_heapless_strings() {
        let expected = "YWFhYWFhYWE=";
        let mut data: Vec<u8, U128> = Vec::new();
        for _ in 0..8 {
            data.push(b'a').unwrap();
        }
        let result = base64_encode::<U128, _>(&data).unwrap();
        assert_eq!(result.as_str(), expected);
    }
    #[test]
    fn byte_encoded_data_can_be_base64_decoded_to_heapless_strings() {
        let expected = "aaaaaaaa";
        let data: [u8; 12] = [
            b'Y', b'W', b'F', b'h', b'Y', b'W', b'F', b'h', b'Y', b'W', b'E', b'=',
        ];

        let result = base64_decode(&data).unwrap();
        let result_string: String<U256> = String::from_utf8(result).unwrap();
        assert_eq!(result_string, expected);
    }
    #[test]
    fn heapless_string_encoded_data_can_be_base64_decoded_to_heapless_strings() {
        let expected = "aaaaaaaa";
        let mut data: String<U128> = String::new();
        data.push_str("YWFhYWFhYWE=").unwrap();

        let result = base64_decode(&data).unwrap();
        let result_string: String<U256> = String::from_utf8(result).unwrap();
        assert_eq!(result_string, expected);
    }

    #[test]
    fn string_slice_encoded_data_can_be_base64_decoded_to_heapless_strings() {
        let expected = "aaaaaaaa";
        let data = "YWFhYWFhYWE=";

        let result = base64_decode(&data).unwrap();
        let result_string: String<U256> = String::from_utf8(result).unwrap();
        assert_eq!(result_string, expected);
    }
}
