extern crate paho_mqtt as mqtt;
use std::time::Duration;
use std::time::{SystemTime, UNIX_EPOCH};
pub const DEFAULT_MQTT_CONNECT_KEEPALIVE_SECONDS: u64 = 240;

use azure_sdk_for_rust_common::error::AZ_ERROR_INSUFFICIENT_SPAN_SIZE;
use heapless::consts::U256;
use heapless::String;

extern crate azure_sdk_for_rust_iot;
use azure_sdk_for_rust_iot::provisioning::client;
use azure_sdk_for_rust_iot::provisioning::common::ProvisioningStatus;
use azure_sdk_for_rust_iot::provisioning::sas;
use azure_sdk_for_rust_iot::provisioning::serialization::{
    DeviceRegistrationResult, ProvisioningServiceErrorDetails, RegistrationOperationStatus,
};
use azure_sdk_for_rust_iot::provisioning::AZ_IOT_PROVISIONING_CLIENT_REGISTER_SUBSCRIBE_TOPIC;

pub fn disconnect_client_from_provisioning_service(mqtt_client: &mqtt::Client) {
    let options = mqtt::DisconnectOptions::default();
    if let Err(err) = mqtt_client.disconnect(options) {
        println!("Error closing MQTT Client: {}", err.to_string());
    }
    println!("Client disconnected from provisioning service.");
}

pub fn subscribe_client_to_provisioning_service_topics(mqtt_client: &mqtt::Client) {
    println!("Subscribing client");
    let subscription = mqtt_client.subscribe(client::Client::get_provisioning_service_topics(), 1);
    if subscription.is_err() {
        panic!("{:?}", subscription.err());
    }
    println!("Client subscribed to provisioning service topics.");
}

// $dps/registrations/PUT/iotdps-register/?$rid=%s
pub fn register_client_with_provisioning_service(mqtt_client: &mqtt::Client) {
    let topic = "$dps/registrations/PUT/iotdps-register/?$rid=1";
    let payload = Vec::new();
    let qos = 1;
    println!("Sending: {}", &topic);
    let message = paho_mqtt::message::Message::new(topic, payload, qos);
    if let Err(response) = mqtt_client.publish(message) {
        panic!("{:?}", response);
    }
    println!("Client registering with provisioning service.");
}
pub fn receive_registration_status(
    receiver_queue: &std::sync::mpsc::Receiver<Option<mqtt::Message>>,
    mqtt_client: &mqtt::Client,
) -> Result<(), &'static str> {
    loop {
        for recieved in receiver_queue {
            if let Some(message) = recieved {
                match process_message(&message, &mqtt_client) {
                    Ok(status) => {
                        if status == ProvisioningStatus::Assigning {
                            continue;
                        } else {
                            return Ok(());
                        }
                    }
                    Err(err) => return Err(err),
                }
            }
        }
    }
}

fn process_message(
    message: &mqtt::Message,
    mqtt_client: &mqtt::Client,
) -> Result<ProvisioningStatus, &'static str> {
    println!("Client received a message from provisioning service.");

    let parsed_topic = parse_topic(message.topic());
    if let (_, Some(delay)) = parsed_topic {
        println!("Retry after {}", delay);
        std::thread::sleep(Duration::from_secs(delay as u64));
    }
    match parsed_topic.0 {
        "$dps/registrations/res/200/?$rid=1" | "$dps/registrations/res/202/?$rid=1" => {
            let register_response = parse_registration_message(&message);
            if register_response.is_err() {
                return Err(register_response.unwrap_err());
            }
            let response = register_response.unwrap();
            println!("operation_id: {}", response.operation_id);
            //println!("operation_status: {:?}", response.operation_status);
            println!("status: {}", response.status);
            if let Ok(status) = response.status.parse::<ProvisioningStatus>() {
                match status {
                    ProvisioningStatus::Assigned => {
                        let state = response.registration_state.unwrap();
                        println!("Client provisioned:");
                        println!("Hub Hostname: {}", state.assigned_hub);
                        println!("Device Id: {}", state.device_id);
                    }
                    ProvisioningStatus::Unassigned
                    | ProvisioningStatus::Failed
                    | ProvisioningStatus::Disabled => {
                        if let Some(state) = response.registration_state {
                            println!("Client provisioning failed:");
                            println!("Registration state: {}", response.status);
                            println!("Last operation status: {}", response.status);
                            println!("Operation ID: {}", response.operation_id);
                            // c code has error_code = extended_error_code / 1000
                            // but the
                            println!("Error code: {}", state.error_code);
                            println!("Error message: {}", state.error_message);
                            println!("Error timestamp: {}", state.last_updated_date_time_utc);
                        }
                    }
                    ProvisioningStatus::Assigning => {
                        az_iot_provisioning_client_query_status_get_publish_topic(
                            &mqtt_client,
                            &response,
                        )?;
                    }
                }
                return Ok(status);
            }
            return Err("Failed to read message");
        }
        topic => {
            println!("Error Topic: {}", topic);
            println!("Payload: {}", message.payload_str());
            let provision_service_error = parse_provision_service_error_message(&message);
            if provision_service_error.is_err() {
                return Err("Could not process error");
            }
            let error = provision_service_error.unwrap();
            println!("Error processing registration");
            print!("Error code: {}", error.error_code);
            if let Some(info) = error.info {
                print!("Info: {}", info);
            }
            print!("Message: {}", error.message);
            print!("Time: {}", error.timestamp_utc);
            print!("Tracking ID: {}", error.tracking_id);
            return Err("Error processing");
        }
    }
}

fn az_iot_provisioning_client_query_status_get_publish_topic(
    mqtt_client: &mqtt::Client,
    response: &RegistrationOperationStatus,
) -> Result<(), &'static str> {
    let topic = client::Client::query_status_get_publish_topic(response.operation_id)?;

    let payload = Vec::new();
    let qos = 1;
    println!("Sending: {}", &topic);
    let message = paho_mqtt::message::Message::new(topic.as_str(), payload, qos);

    if mqtt_client.publish(message).is_err() {
        return Err("Failed to publish message");
    }
    Ok(())
}
fn parse_provision_service_error_message<'a>(
    message: &'a mqtt::Message,
) -> Result<ProvisioningServiceErrorDetails<'a>, &'static str> {
    let payload: &[u8] = message.payload();
    let r = serde_json_core::from_slice::<ProvisioningServiceErrorDetails<'_>>(payload);

    if r.is_err() {
        return Err("couldn't parse body");
    }
    let result = r.unwrap();
    Ok(result)
}
fn parse_registration_message<'a>(
    message: &'a mqtt::Message,
) -> Result<RegistrationOperationStatus<'a>, &'static str> {
    let (topic, retry) = parse_topic(message.topic());
    let duration = parse_duration(retry);
    let payload: &[u8] = message.payload();
    let r = serde_json_core::from_slice::<RegistrationOperationStatus<'_>>(payload);

    if r.is_err() {
        return Err("couldn't parse body");
    }
    let result = r.unwrap();
    Ok(result)
}

fn parse_duration(value: Option<u64>) -> Option<Duration> {
    if let Some(time) = value {
        return Some(Duration::from_secs(time));
    }
    None
}
fn parse_topic(topic: &str) -> (&str, Option<u64>) {
    if let Some(index) = topic.find('&') {
        // TODO: this can panic on marlformed url
        let lhs = &topic[0..index];
        let rhs = &topic[index + 1..];
        if let Some(index) = rhs.find('=') {
            let retry = u64::from_str_radix(&rhs[index + 1..], 10);
            if let Ok(delay) = retry {
                return (lhs, Some(delay));
            }
        }
    }
    return (topic, None);
}

pub fn get_epoch_expiration_time_from_hours(hours: u64) -> u64 {
    match SystemTime::now().duration_since(UNIX_EPOCH) {
        Ok(n) => return n.as_secs() + hours * 60 * 60,
        Err(_) => panic!(),
    }
}

pub fn connect_client_to_provisioning_service(
    client: &client::Client,
    sas_key: &str,
    trust_store: Option<&str>,
) -> mqtt::client::Client {
    let client_id = client.get_client_id();
    let user_name = client.get_user_name().unwrap();
    let sas_duration = get_epoch_expiration_time_from_hours(1);

    let sas_password = sas::get_password(&client, &sas_key, sas_duration, None).unwrap();

    let create_opts = mqtt::CreateOptionsBuilder::new()
        .client_id(client_id)
        .server_uri(client.global_device_endpoint)
        .persistence(mqtt::PersistenceType::None)
        .finalize();

    let mut ssl_options_builder = mqtt::SslOptionsBuilder::new();
    if let Some(trust) = trust_store {
        ssl_options_builder.trust_store(trust);
    }
    let ssl_options = ssl_options_builder.finalize();

    let conn_opts = mqtt::ConnectOptionsBuilder::new()
        .keep_alive_interval(Duration::from_secs(20))
        .user_name(user_name.as_str())
        .password(sas_password.as_str())
        .clean_session(false)
        .keep_alive_interval(std::time::Duration::from_secs(
            DEFAULT_MQTT_CONNECT_KEEPALIVE_SECONDS,
        ))
        .ssl_options(ssl_options)
        .finalize();

    let mqtt_client = mqtt::Client::new(create_opts).expect("Could not create MQTT Client");

    mqtt_client
        .connect(conn_opts)
        .expect("MQTT Client could not connect.");
    println!("Client connected to {}.", client.global_device_endpoint);
    return mqtt_client;
}
