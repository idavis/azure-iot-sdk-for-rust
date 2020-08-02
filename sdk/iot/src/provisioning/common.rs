use core::str::FromStr;

use crate::provisioning::error::{Error, ErrorKind};

use heapless::consts::{U128, U256};
use heapless::{String, Vec};

use serde::{Deserialize, Serialize};

//from edgelet-core
pub trait CoreProvisioningResult {
    fn device_id(&self) -> &str;
    fn hub_name(&self) -> &str;
}

#[derive(Clone, Copy, Serialize, Deserialize, Debug, PartialEq)]
pub enum ProvisioningStatus {
    Assigned,
    Assigning,
    Disabled,
    Failed,
    Unassigned,
}

impl ProvisioningStatus {
    #[inline]
    pub fn is_complete(self) -> bool {
        match self {
            ProvisioningStatus::Unassigned | ProvisioningStatus::Assigning => false,
            _ => true,
        }
    }
}

impl From<&str> for ProvisioningStatus {
    fn from(s: &str) -> ProvisioningStatus {
        // TODO: check with DPS substatus value for DeviceDataUpdated when it is implemented on service side
        match s {
            "assigned" => ProvisioningStatus::Assigned,
            "assigning" => ProvisioningStatus::Assigning,
            "disabled" => ProvisioningStatus::Disabled,
            "failed" => ProvisioningStatus::Failed,
            "unassigned" => ProvisioningStatus::Unassigned,
            _ => {
                //debug!("Provisioning result substatus {}", s);
                panic!("Should never hit. TODO: see what desired behavior should be.");
            }
        }
    }
}

#[derive(Clone, Copy, Serialize, Deserialize, Debug, PartialEq)]
pub enum ReprovisioningStatus {
    DeviceDataNotUpdated,
    DeviceDataUpdated,
    InitialAssignment,
    DeviceDataMigrated,
    DeviceDataReset,
}

//#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SymmetricKeyCredential {
    //#[serde(skip_serializing_if = "Option::is_none")]
    key: Option<Vec<u8, U128>>,
}

impl SymmetricKeyCredential {
    pub fn new(key: Vec<u8, U128>) -> Self {
        SymmetricKeyCredential { key: Some(key) }
    }

    pub fn key(&self) -> Option<&[u8]> {
        self.key.as_ref().map(AsRef::as_ref)
    }
}

//#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct X509Credential {
    identity_cert: String<U256>,
    identity_private_key: String<U256>,
}

impl X509Credential {
    pub fn new(identity_cert: String<U256>, identity_private_key: String<U256>) -> Self {
        X509Credential {
            identity_cert,
            identity_private_key,
        }
    }

    pub fn identity_cert(&self) -> &str {
        self.identity_cert.as_str()
    }

    pub fn identity_private_key(&self) -> &str {
        self.identity_private_key.as_str()
    }
}

//#[derive(Clone, Serialize, Deserialize, Debug)]
pub enum AuthType {
    SymmetricKey(SymmetricKeyCredential),
    X509(X509Credential),
}

#[derive(Clone, Copy, Serialize, Deserialize, Debug, PartialEq)]
pub enum CredentialSource {
    Payload,
    Hsm,
}

//#[derive(Clone, Debug, Deserialize, Serialize)]
//#[derive(Clone, Debug)]
pub struct Credentials {
    auth_type: AuthType,
    source: CredentialSource,
}
#[cfg(test)]
mod credential_tests {
    use super::*;
    #[test]
    fn _test() {
        let key: Vec<u8, U128> = Vec::new();
        let auth_type = AuthType::SymmetricKey(SymmetricKeyCredential::new(key));
        Credentials::new(auth_type, CredentialSource::Payload);
    }
}
impl Credentials {
    pub fn new(auth_type: AuthType, source: CredentialSource) -> Self {
        Credentials { auth_type, source }
    }

    pub fn auth_type(&self) -> &AuthType {
        &self.auth_type
    }

    pub fn source(&self) -> &CredentialSource {
        &self.source
    }
}

impl From<&str> for ReprovisioningStatus {
    fn from(s: &str) -> ReprovisioningStatus {
        // TODO: check with DPS substatus value for DeviceDataUpdated when it is implemented on service side
        match s {
            "deviceDataMigrated" => ReprovisioningStatus::DeviceDataMigrated,
            "deviceDataReset" => ReprovisioningStatus::DeviceDataReset,
            "initialAssignment" => ReprovisioningStatus::InitialAssignment,
            _ => {
                //debug!("Provisioning result substatus {}", s);
                ReprovisioningStatus::InitialAssignment
            }
        }
    }
}

impl Default for ReprovisioningStatus {
    fn default() -> Self {
        ReprovisioningStatus::InitialAssignment
    }
}

impl FromStr for ProvisioningStatus {
    type Err = crate::provisioning::error::Error;

    fn from_str(s: &str) -> Result<ProvisioningStatus, Self::Err> {
        match s {
            "assigned" => Ok(ProvisioningStatus::Assigned),
            "assigning" => Ok(ProvisioningStatus::Assigning),
            "disabled" => Ok(ProvisioningStatus::Disabled),
            "failed" => Ok(ProvisioningStatus::Failed),
            "unassigned" => Ok(ProvisioningStatus::Unassigned),
            _ => Err(Error::from(ErrorKind::InvalidProvisioningStatus)),
        }
    }
}

#[cfg(test)]
mod tests_completion {
    use super::*;
    #[test]
    fn unassigned_is_not_complete() {
        assert!(!ProvisioningStatus::Unassigned.is_complete());
    }

    #[test]
    fn assigning_is_not_complete() {
        assert!(!ProvisioningStatus::Assigning.is_complete());
    }

    #[test]
    fn assigned_is_not_complete() {
        assert!(ProvisioningStatus::Assigned.is_complete());
    }

    #[test]
    fn failed_is_complete() {
        assert!(ProvisioningStatus::Failed.is_complete());
    }

    #[test]
    fn disabled_is_complete() {
        assert!(ProvisioningStatus::Disabled.is_complete());
    }
}
