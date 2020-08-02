use hmac::{Hmac, Mac, NewMac};
use sha2::Sha256;
// Create alias for HMAC-SHA256
type HmacSha256 = Hmac<Sha256>;

use heapless::consts::U128;

use heapless::consts::U256;
use heapless::String;
use heapless::Vec;

use super::base64::{base64_decode, base64_encode};
use super::util::u64_to_string;

use azure_sdk_for_rust_common::error::AZ_ERROR_INSUFFICIENT_SPAN_SIZE;
const LF: char = '\n';
const AMPERSAND: char = '&';
const EQUAL_SIGN: char = '=';
const SCOPE_REGISTRATIONS_STRING: &str = "%2fregistrations%2f";
const SAS_TOKEN_SR: &str = "SharedAccessSignature sr";
const SAS_TOKEN_SE: &str = "se";
const SAS_TOKEN_SIG: &str = "sig";
const SAS_TOKEN_SKN: &str = "skn";
use super::percent_encode;

// Concatenates:
// "SharedAccessSignature sr=<url-encoded(resource-string)>&sig=<signature>&se=<expiration-time>"
// plus, if key_name is not NULL, "&skn=<key-name>"
//
// Where:
// resource-string: <scope-id>/registrations/<registration-id>
pub fn get_password(
    client: &super::client::Client<'_>,
    sas_key: &str,
    token_expiration_epoch_time: u64,
    key_name: Option<&String<U128>>,
) -> Result<String<U256>, &'static str> {
    let sas_signature = get_sas_get_signature(&client, token_expiration_epoch_time)?;
    let sas_b64_encoded_hmac256_signed_signature: String<U256> =
        get_sas_b64_encoded_hmac256_signed_signature(&sas_key, &sas_signature)?;

    let mut res: String<U256> = String::new();
    if res.push_str(SAS_TOKEN_SR).is_err() {
        return Err(AZ_ERROR_INSUFFICIENT_SPAN_SIZE);
    }
    if res.push(EQUAL_SIGN).is_err() {
        return Err(AZ_ERROR_INSUFFICIENT_SPAN_SIZE);
    }
    let encoded_scope = percent_encode::encode(client.id_scope)?;
    if res.push_str(encoded_scope.as_str()).is_err() {
        return Err(AZ_ERROR_INSUFFICIENT_SPAN_SIZE);
    }
    if res.push_str(SCOPE_REGISTRATIONS_STRING).is_err() {
        return Err(AZ_ERROR_INSUFFICIENT_SPAN_SIZE);
    }
    let encoded_reg_id = percent_encode::encode(client.registration_id)?;
    if res.push_str(encoded_reg_id.as_str()).is_err() {
        return Err(AZ_ERROR_INSUFFICIENT_SPAN_SIZE);
    }
    if res.push(AMPERSAND).is_err() {
        return Err(AZ_ERROR_INSUFFICIENT_SPAN_SIZE);
    }
    if res.push_str(SAS_TOKEN_SIG).is_err() {
        return Err(AZ_ERROR_INSUFFICIENT_SPAN_SIZE);
    }
    if res.push(EQUAL_SIGN).is_err() {
        return Err(AZ_ERROR_INSUFFICIENT_SPAN_SIZE);
    }
    let encoded_base64_hmac_sha256_signature =
        percent_encode::encode(sas_b64_encoded_hmac256_signed_signature.as_str())?;
    if res
        .push_str(encoded_base64_hmac_sha256_signature.as_str())
        .is_err()
    {
        return Err(AZ_ERROR_INSUFFICIENT_SPAN_SIZE);
    }
    if res.push(AMPERSAND).is_err() {
        return Err(AZ_ERROR_INSUFFICIENT_SPAN_SIZE);
    }
    if res.push_str(SAS_TOKEN_SE).is_err() {
        return Err(AZ_ERROR_INSUFFICIENT_SPAN_SIZE);
    }
    if res.push(EQUAL_SIGN).is_err() {
        return Err(AZ_ERROR_INSUFFICIENT_SPAN_SIZE);
    }
    let epoch_string = u64_to_string(token_expiration_epoch_time);
    if res.push_str(epoch_string.as_str()).is_err() {
        return Err(AZ_ERROR_INSUFFICIENT_SPAN_SIZE);
    }
    if let Some(key) = key_name {
        if res.push(AMPERSAND).is_err() {
            return Err(AZ_ERROR_INSUFFICIENT_SPAN_SIZE);
        }
        if res.push_str(SAS_TOKEN_SKN).is_err() {
            return Err(AZ_ERROR_INSUFFICIENT_SPAN_SIZE);
        }
        if res.push(EQUAL_SIGN).is_err() {
            return Err(AZ_ERROR_INSUFFICIENT_SPAN_SIZE);
        }
        if res.push_str(key.as_str()).is_err() {
            return Err(AZ_ERROR_INSUFFICIENT_SPAN_SIZE);
        }
    }
    res.truncate(res.len());
    Ok(res)
}

// Produces the following signature:
// url-encoded(<resource-string>)\n<expiration-time>
// Where
// resource-string: <scope-id>/registrations/<registration-id>
fn get_sas_get_signature(
    client: &super::client::Client<'_>,
    token_expiration_epoch_time: u64,
) -> Result<String<U128>, &'static str> {
    let mut res: String<U128> = String::new();
    let encoded_scope = percent_encode::encode(client.id_scope)?;
    if res.push_str(encoded_scope.as_str()).is_err() {
        return Err(AZ_ERROR_INSUFFICIENT_SPAN_SIZE);
    }
    if res.push_str(SCOPE_REGISTRATIONS_STRING).is_err() {
        return Err(AZ_ERROR_INSUFFICIENT_SPAN_SIZE);
    }
    let encoded_reg_id = percent_encode::encode(client.registration_id)?;
    if res.push_str(encoded_reg_id.as_str()).is_err() {
        return Err(AZ_ERROR_INSUFFICIENT_SPAN_SIZE);
    }
    if res.push(LF).is_err() {
        return Err(AZ_ERROR_INSUFFICIENT_SPAN_SIZE);
    }
    let epoch_string = u64_to_string(token_expiration_epoch_time);
    if res.push_str(epoch_string.as_str()).is_err() {
        return Err(AZ_ERROR_INSUFFICIENT_SPAN_SIZE);
    }
    res.truncate(res.len());
    Ok(res)
}

fn get_sas_b64_encoded_hmac256_signed_signature(
    sas_key: &str,
    sas_sig: &String<U128>,
) -> Result<String<U256>, &'static str> {
    let decoded_sas_key = base64_decode(&sas_key)?;
    let sas_encoded_hmac256_signed_signature = hmac_sha256_sign(&decoded_sas_key, &sas_sig)?;
    let sas_b64_encoded_hmac256_signed_signature =
        base64_encode(&sas_encoded_hmac256_signed_signature)?;
    Ok(sas_b64_encoded_hmac256_signed_signature)
}

fn hmac_sha256_sign<'a>(
    key: &Vec<u8, U256>,
    data: &String<U128>,
) -> Result<Vec<u8, U128>, &'a str> {
    let mut mac = HmacSha256::new_varkey(key).expect("HMAC can take key of any size");
    mac.update(data.as_bytes());

    // `result` has type `Output` which is a thin wrapper around array of
    // bytes for providing constant time equality check
    let result = mac.finalize();

    // To get underlying array use `into_bytes` method, but be careful, since
    // incorrect use of the code value may permit timing attacks which defeat
    // the security provided by the `Output`
    let code_bytes = result.into_bytes();
    let mut vec: Vec<u8, U128> = Vec::new();
    for b in code_bytes.iter() {
        if vec.push(*b).is_err() {
            return Err(AZ_ERROR_INSUFFICIENT_SPAN_SIZE);
        }
    }
    Ok(vec)
}

#[cfg(test)]
mod tests_sas_b64_encoded_hmac256_signed_signature {
    use super::*;
    use crate::provisioning::client::Client;
    #[test]
    fn signature_is_generated_correctly_for_fixed_input() {
        let global_device_endpoint = "ssl://global.azure-devices-provisioning.net:8883";
        let id_scope = "eight675309";
        let registration_id = "1-1-2-3-5-8-13-21";
        let sas_key = "VGhpcyB0aGluZyBhbGwgdGhpbmdzIGl0IGRldm91cnM=";
        let token_expiration_epoch_time = 1_596_897_539;
        let options = None;
        let client = Client::new(
            &global_device_endpoint,
            &id_scope,
            &registration_id,
            options,
        );
        let sas_signature = get_sas_get_signature(&client, token_expiration_epoch_time).unwrap();
        let base64_hmac_sha256_signature: String<U256> =
            get_sas_b64_encoded_hmac256_signed_signature(&sas_key, &sas_signature).unwrap();
        let expected = "npj3I09+Jtl6VYHZM/H5mKMG8jn4Y3zty3dMjMkMMDs=";
        assert_eq!(base64_hmac_sha256_signature.as_str(), expected);
    }
}
