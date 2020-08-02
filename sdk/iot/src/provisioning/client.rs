pub struct ClientOptions<'a> {
    pub user_agent: &'a str,
}

pub struct Client<'a> {
    pub global_device_endpoint: &'a str,
    pub id_scope: &'a str,
    pub registration_id: &'a str,
    pub options: ClientOptions<'a>,
}

impl<'a> Default for ClientOptions<'a> {
    #[inline]
    fn default() -> ClientOptions<'a> {
        ClientOptions { user_agent: "" }
    }
}

impl<'a> Client<'a> {
    pub fn new(
        global_device_endpoint: &'a str,
        id_scope: &'a str,
        registration_id: &'a str,
        options: Option<ClientOptions<'a>>,
    ) -> Client<'a> {
        Client {
            global_device_endpoint,
            id_scope,
            registration_id,
            options: options.unwrap_or_default(),
        }
    }
}

//use common::error::Error;
use super::{CLIENT_REGISTER_SUBSCRIBE_TOPIC, SERVICE_VERSION, STR_GET_IOTDPS_GET_OPERATIONSTATUS};

use heapless::consts::{U128, U256};
use heapless::String;

use azure_sdk_for_rust_common::error::AZ_ERROR_INSUFFICIENT_SPAN_SIZE;

impl<'a> Client<'a> {
    pub fn get_client_id(&self) -> &'a str {
        self.registration_id
    }
    // <id_scope>/registrations/<registration_id>/api-version=<service_version>
    pub fn get_user_name(&self) -> Result<String<U128>, &'static str> {
        let str_registrations = Client::get_registrations();

        let mut res: String<U128> = String::new();

        if res.push_str(&self.id_scope).is_err() {
            return Err(AZ_ERROR_INSUFFICIENT_SPAN_SIZE);
        }
        if res.push_str(&str_registrations).is_err() {
            return Err(AZ_ERROR_INSUFFICIENT_SPAN_SIZE);
        }
        if res.push_str(&self.registration_id).is_err() {
            return Err(AZ_ERROR_INSUFFICIENT_SPAN_SIZE);
        }
        if res.push_str("/api-version=").is_err() {
            return Err(AZ_ERROR_INSUFFICIENT_SPAN_SIZE);
        }
        if res.push_str(&SERVICE_VERSION).is_err() {
            return Err(AZ_ERROR_INSUFFICIENT_SPAN_SIZE);
        }
        if !self.options.user_agent.is_empty() {
            if res.push_str("&ClientVersion=").is_err() {
                return Err(AZ_ERROR_INSUFFICIENT_SPAN_SIZE);
            }
            if res.push_str(&self.options.user_agent).is_err() {
                return Err(AZ_ERROR_INSUFFICIENT_SPAN_SIZE);
            }
        }
        res.truncate(res.len());
        Ok(res)
    }

    // Topic: $dps/registrations/GET/iotdps-get-operationstatus/?$rid=1&operationId=%s
    pub fn query_status_get_publish_topic(
        operation_id: &str,
    ) -> Result<String<U256>, &'static str> {
        let mut topic: String<U256> = String::new();
        if topic.push_str(Client::get_dps_registrations()).is_err() {
            return Err(AZ_ERROR_INSUFFICIENT_SPAN_SIZE);
        }
        if topic.push_str(STR_GET_IOTDPS_GET_OPERATIONSTATUS).is_err() {
            return Err(AZ_ERROR_INSUFFICIENT_SPAN_SIZE);
        }
        if topic.push_str(operation_id).is_err() {
            return Err(AZ_ERROR_INSUFFICIENT_SPAN_SIZE);
        }
        topic.truncate(topic.len());
        Ok(topic)
    }
    // $dps/registrations/res/
    pub fn get_dps_registrations_res() -> &'static str {
        &CLIENT_REGISTER_SUBSCRIBE_TOPIC[0..23]
    }

    // /registrations/
    pub fn get_registrations() -> &'static str {
        &CLIENT_REGISTER_SUBSCRIBE_TOPIC[4..19]
    }

    // $dps/registrations/
    pub fn get_dps_registrations() -> &'static str {
        &CLIENT_REGISTER_SUBSCRIBE_TOPIC[0..19]
    }

    // "$dps/registrations/res/#"
    pub fn get_provisioning_service_topics() -> &'static str {
        return CLIENT_REGISTER_SUBSCRIBE_TOPIC;
    }

    // GET/iotdps-get-operationstatus/?$rid=1&operationId=
    pub fn get_iotdps_get_operationstatus() -> &'static str {
        return STR_GET_IOTDPS_GET_OPERATIONSTATUS;
    }
}

#[cfg(test)]
mod tests_new {
    use super::*;
    #[test]
    fn global_device_endpoint_flows_through() {
        assert_eq!(
            Client::new("gde", "", "", None).global_device_endpoint,
            "gde"
        );
    }
    #[test]
    fn id_scope_flows_through() {
        assert_eq!(Client::new("", "ids", "", None).id_scope, "ids");
    }
    #[test]
    fn registration_id_flows_through() {
        assert_eq!(Client::new("", "", "rid", None).registration_id, "rid");
    }
    #[test]
    fn options_defaults_flows_through() {
        assert_eq!(
            Client::new(
                "",
                "",
                "",
                Some(ClientOptions {
                    user_agent: "agent"
                })
            )
            .options
            .user_agent,
            "agent"
        );
    }
    #[test]
    fn options_defaults_on_none() {
        assert_eq!(
            Client::new("", "", "", None).options.user_agent,
            ClientOptions::default().user_agent
        );
    }
}

#[cfg(test)]
mod tests_consts {
    use super::*;
    #[test]
    fn unassigned_is_not_complete() {
        assert_eq!(
            Client::get_dps_registrations_res(),
            "$dps/registrations/res/"
        );
    }

    #[test]
    fn _az_iot_provisioning_get_str_registrations() {
        assert_eq!(Client::get_registrations(), "/registrations/");
    }

    #[test]
    fn _az_iot_provisioning_get_str_dps_registrations() {
        assert_eq!(Client::get_dps_registrations(), "$dps/registrations/");
    }
}
