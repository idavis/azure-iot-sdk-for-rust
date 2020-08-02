mod base64;
pub mod certificate;
pub mod client;
pub mod common;
pub mod error;
mod percent_encode;
pub mod sas;
pub mod serialization;
mod util;

pub const SERVICE_VERSION: &str = "2019-03-31";
pub const CLIENT_REGISTER_SUBSCRIBE_TOPIC: &str = "$dps/registrations/res/#";
pub const STR_PUT_IOTDPS_REGISTER: &str = "PUT/iotdps-register/?$rid=1";
pub const STR_GET_IOTDPS_GET_OPERATIONSTATUS: &str =
    "GET/iotdps-get-operationstatus/?$rid=1&operationId=";
pub const AZ_IOT_PROVISIONING_CLIENT_REGISTER_SUBSCRIBE_TOPIC: &str = "$dps/registrations/res/#";

//static const az_span str_put_iotdps_register
//    = AZ_SPAN_LITERAL_FROM_STR("PUT/iotdps-register/?$rid=1");
