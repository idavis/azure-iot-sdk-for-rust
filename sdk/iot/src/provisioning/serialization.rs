use serde::{Deserialize, Serialize};

// https://docs.microsoft.com/en-us/rest/api/iot-dps/runtimeregistration/registerdevice#deviceregistrationresult

// PUT https://global.azure-devices-provisioning.net/{idScope}/registrations/{registrationId}/register?api-version=2019-03-31

// https://docs.microsoft.com/en-us/rest/api/iot-dps/runtimeregistration/registerdevice#deviceregistration
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct DeviceRegistration<'a> {
    /// Custom allocation payload.
    #[serde(rename = "payload")]
    pub payload: &'a str,
    /// Registration Id.
    #[serde(rename = "registrationId")]
    pub registration_id: &'a str,
    #[serde(rename = "status")]
    pub tpm: Option<TpmAttestation<'a>>,
}

impl<'a> DeviceRegistration<'a> {
    pub fn new(
        payload: &'a str,
        registration_id: &'a str,
        tpm: Option<TpmAttestation<'a>>,
    ) -> DeviceRegistration<'a> {
        DeviceRegistration {
            payload,
            registration_id,
            tpm,
        }
    }
}

// https://docs.microsoft.com/en-us/rest/api/iot-dps/runtimeregistration/registerdevice#deviceregistrationresult
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct DeviceRegistrationResult<'a> {
    #[serde(rename = "assignedHub", default)]
    pub assigned_hub: &'a str,
    #[serde(rename = "createdDateTimeUtc", default)]
    pub created_date_time_utc: &'a str,
    #[serde(rename = "deviceId", default)]
    pub device_id: &'a str,

    #[serde(rename = "errorCode", default)]
    pub error_code: &'a str,
    #[serde(rename = "errorMessage", default)]
    pub error_message: &'a str,

    #[serde(rename = "etag", default)]
    pub etag: &'a str,

    #[serde(rename = "lastUpdatedDateTimeUtc", default)]
    pub last_updated_date_time_utc: &'a str,

    /// Custom allocation payload returned from the webhook to the device.
    #[serde(rename = "payload", default)]
    pub payload: &'a str,

    /// The registration ID is alphanumeric, lowercase, and may contain hyphens.
    #[serde(rename = "registrationId", default)]
    pub registration_id: &'a str,

    /// Enrollment status. => common::ProvisioningStatus
    #[serde(rename = "status", default)]
    pub status: &'a str,

    /// Substatus for 'Assigned' devices. Possible values include
    /// - 'initialAssignment':  Device has been assigned to an IoT hub for the first time
    /// - 'deviceDataMigrated': Device has been assigned to a different IoT hub and its
    ///                         device data was migrated from the previously assigned IoT hub.
    ///                         Device data was removed from the previously assigned IoT hub
    /// - 'deviceDataReset':    Device has been assigned to a different IoT hub and its device
    ///                         data was populated from the initial state stored in the enrollment.
    ///                         Device data was removed from the previously assigned IoT hub.
    #[serde(rename = "substatus", default)]
    pub substatus: &'a str,

    #[serde(
        rename = "symmetricKey",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub symmetric_key: Option<SymmetricKeyRegistrationResult<'a>>,
    #[serde(rename = "tpm", skip_serializing_if = "Option::is_none", default)]
    pub tpm: Option<TpmRegistrationResult<'a>>,
    #[serde(rename = "x509", skip_serializing_if = "Option::is_none", default)]
    pub x509: Option<X509RegistrationResult<'a>>,
}

impl<'a> DeviceRegistrationResult<'a> {
    pub fn new(
        assigned_hub: &'a str,
        created_date_time_utc: &'a str,
        device_id: &'a str,
        error_code: &'a str,
        error_message: &'a str,
        etag: &'a str,
        last_updated_date_time_utc: &'a str,
        payload: &'a str,
        registration_id: &'a str,
        status: &'a str,
        substatus: &'a str,
        symmetric_key: Option<SymmetricKeyRegistrationResult<'a>>,
        tpm: Option<TpmRegistrationResult<'a>>,
        x509: Option<X509RegistrationResult<'a>>,
    ) -> DeviceRegistrationResult<'a> {
        DeviceRegistrationResult {
            assigned_hub,
            created_date_time_utc,
            device_id,
            error_code,
            error_message,
            etag,
            last_updated_date_time_utc,
            payload,
            registration_id,
            status,
            substatus,
            symmetric_key,
            tpm,
            x509,
        }
    }
}
// https://docs.microsoft.com/en-us/rest/api/iot-dps/runtimeregistration/registerdevice#provisioningserviceerrordetails
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct ProvisioningServiceErrorDetails<'a> {
    #[serde(rename = "errorCode", default)]
    pub error_code: u32,
    #[serde(rename = "info", skip_serializing_if = "Option::is_none", default)]
    pub info: Option<&'a str>, // object
    #[serde(rename = "message", default)]
    pub message: &'a str,
    #[serde(rename = "timestampUtc", default)]
    pub timestamp_utc: &'a str,
    #[serde(rename = "trackingId", default)]
    pub tracking_id: &'a str,
}

#[cfg(test)]
mod provisioning_service_error_details_serialization_tests {
    use super::*;
    #[test]
    fn smoke() {
        let source = "{\"errorCode\":401000,\"trackingId\":\"4e4fbac4-a5f1-4ad4-ae4c-2485742aadd4\",\"message\":\"Unauthorized\",\"timestampUtc\":\"2020-08-08T13:16:50.5067952Z\"}";
        let slice = source.as_bytes();
        let object = serde_json_core::from_slice::<ProvisioningServiceErrorDetails<'_>>(slice);
        assert!(object.is_ok());
        let actual = object.unwrap();
        assert_eq!(401000, actual.error_code);
        assert!(actual.info.is_none());
        assert_eq!("Unauthorized", actual.message);
        assert_eq!("2020-08-08T13:16:50.5067952Z", actual.timestamp_utc);
        assert_eq!("4e4fbac4-a5f1-4ad4-ae4c-2485742aadd4", actual.tracking_id);
    }
}

// https://docs.microsoft.com/en-us/rest/api/iot-dps/runtimeregistration/registerdevice#registrationoperationstatus
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct RegistrationOperationStatus<'a> {
    /// Operation ID.
    #[serde(rename = "operationId")]
    pub operation_id: &'a str,
    /// Device registration status.
    #[serde(
        rename = "registrationState",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub registration_state: Option<DeviceRegistrationResult<'a>>,
    /// Device enrollment status. => common::ProvisioningStatus
    #[serde(rename = "status")]
    pub status: &'a str,
}

impl<'a> RegistrationOperationStatus<'a> {
    pub fn new(
        status: &'a str,
        operation_id: &'a str,
        registration_state: Option<DeviceRegistrationResult<'a>>,
    ) -> RegistrationOperationStatus<'a> {
        RegistrationOperationStatus {
            status,
            operation_id,
            registration_state,
        }
    }
}

// https://docs.microsoft.com/en-us/rest/api/iot-dps/runtimeregistration/registerdevice#symmetrickeyregistrationresult
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct SymmetricKeyRegistrationResult<'a> {
    #[serde(rename = "enrollmentGroupId", default)]
    enrollment_group_id: &'a str,
}

// https://docs.microsoft.com/en-us/rest/api/iot-dps/runtimeregistration/registerdevice#tpmattestation
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct TpmAttestation<'a> {
    #[serde(rename = "endorsementKey")]
    pub endorsement_key: &'a str,
    #[serde(rename = "storageRootKey")]
    pub storage_root_key: &'a str,
}

// https://docs.microsoft.com/en-us/rest/api/iot-dps/runtimeregistration/registerdevice#tpmregistrationresult
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct TpmRegistrationResult<'a> {
    #[serde(rename = "authenticationKey", default)]
    authentication_key: &'a str,
}

// https://docs.microsoft.com/en-us/rest/api/iot-dps/runtimeregistration/registerdevice#x509certificateinfo
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct X509CertificateInfo<'a> {
    #[serde(rename = "issuerName")]
    issuer_name: &'a str,
    #[serde(rename = "notAfterUtc")]
    not_after_utc: &'a str,
    #[serde(rename = "notBeforeUtc")]
    not_before_utc: &'a str,
    #[serde(rename = "serialNumber")]
    serial_number: &'a str,
    #[serde(rename = "sha1Thumbprint")]
    sha1_thumbprint: &'a str,
    #[serde(rename = "sha256Thumbprint")]
    sha256_thumbprint: &'a str,
    #[serde(rename = "subjectName")]
    subject_name: &'a str,
    #[serde(rename = "version")]
    version: u32,
}

// https://docs.microsoft.com/en-us/rest/api/iot-dps/runtimeregistration/registerdevice#x509registrationresult
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct X509RegistrationResult<'a> {
    #[serde(rename = "certificateInfo")]
    certificate_info: X509CertificateInfo<'a>,
    #[serde(rename = "enrollmentGroupId")]
    enrollment_group_id: &'a str,
    #[serde(rename = "signingCertificateInfo")]
    signing_certificate_info: X509CertificateInfo<'a>,
}

#[cfg(test)]
mod tests_serialization_round_trip {
    use super::*;
    use heapless::consts::U256;
    #[test]
    fn unassigned_is_not_complete() {
        let source = RegistrationOperationStatus::new("some status", "assigned", None);
        let ser: heapless::String<U256> = serde_json_core::to_string::<U256, _>(&source).unwrap();
        //assert!(!ser.is_err());
        let sr = ser.as_str();
        let copy = serde_json_core::from_str::<RegistrationOperationStatus<'_>>(sr);
        assert!(!copy.is_err());
        let result = copy.unwrap();
        assert_eq!(source, result);
    }
}

#[cfg(test)]
mod register_response_serialization_tests {
    use super::*;
    #[test]
    fn register_response_without_registration_state_deserializes() {
        let source = "{\"operationId\":\"4.214465a7b4233f53.e65f9871-d30c-47b1-8889-c8b99e24f9d1\",\"status\":\"assigned\"}";
        let deserialized =
            serde_json_core::from_str::<RegistrationOperationStatus<'_>>(source).unwrap();

        assert_eq!(
            "4.214465a7b4233f53.e65f9871-d30c-47b1-8889-c8b99e24f9d1",
            deserialized.operation_id
        );
        assert_eq!("assigned", deserialized.status);
        assert!(deserialized.registration_state.is_none());
    }

    #[test]
    fn register_response_with_registration_state_deserializes() {
        let source = "{\"operationId\":\"4.214465a7b4233f53.e65f9871-d30c-47b1-8889-c8b99e24f9d1\",\"status\":\"assigned\",\"registrationState\":{\"registrationId\":\"test\",\"createdDateTimeUtc\":\"2020-08-04T21:39:08.4834929Z\",\"assignedHub\":\"example.azure-devices.net\",\"deviceId\":\"1-1-2-3-5-8-13\",\"status\":\"assigned\",\"substatus\":\"initialAssignment\",\"lastUpdatedDateTimeUtc\":\"2020-08-04T21:39:08.6951685Z\",\"etag\":\"ImY3MDEyN2YxLTAwMDAtMDgwMC0wMDAwLTVmMjlkNTdjMDAwMCI=\"}}";
        let deserialized =
            serde_json_core::from_str::<RegistrationOperationStatus<'_>>(source).unwrap();

        assert_eq!(
            "4.214465a7b4233f53.e65f9871-d30c-47b1-8889-c8b99e24f9d1",
            deserialized.operation_id
        );
        assert_eq!("assigned", deserialized.status);
        assert!(deserialized.registration_state.is_some());
        let state = deserialized.registration_state.unwrap();
        assert_eq!("example.azure-devices.net", state.assigned_hub);
        assert_eq!("2020-08-04T21:39:08.4834929Z", state.created_date_time_utc);
        assert_eq!("1-1-2-3-5-8-13", state.device_id);
        assert_eq!(
            "ImY3MDEyN2YxLTAwMDAtMDgwMC0wMDAwLTVmMjlkNTdjMDAwMCI=",
            state.etag
        );
        assert_eq!(
            "2020-08-04T21:39:08.6951685Z",
            state.last_updated_date_time_utc
        );
        assert_eq!("test", state.registration_id);
        assert_eq!("assigned", state.status);
        assert_eq!("initialAssignment", state.substatus);
    }
}
