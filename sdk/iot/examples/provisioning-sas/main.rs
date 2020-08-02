extern crate paho_mqtt as mqtt;
use std::process;
pub const DEFAULT_MQTT_CONNECT_PORT: i32 = 8883;
pub const DEFAULT_MQTT_CONNECT_KEEPALIVE_SECONDS: u64 = 240;

extern crate azure_sdk_for_rust_iot;
use azure_sdk_for_rust_iot::provisioning::client;

mod lib;
use lib::*;

macro_rules! env_require {
    ($str: tt) => {{
        option_env!($str).expect(concat!($str, " must be set before running."))
    }};
}

fn main() {
    let global_provisioning_endpoint = option_env!("ENV_GLOBAL_PROVISIONING_ENDPOINT")
        .unwrap_or("ssl://global.azure-devices-provisioning.net:8883");

    let id_scope = env_require!("AZ_IOT_ID_SCOPE");

    let registration_id = env_require!("AZ_IOT_REGISTRATION_ID_SAS");

    let sas_key = env_require!("AZ_IOT_PROVISIONING_SAS_KEY");

    let sas_key_duration =
        option_env!("AZ_IOT_PROVISIONING_SAS_KEY_DURATION_MINUTES").unwrap_or("120");

    let trust_store = option_env!("AZ_IOT_DEVICE_X509_TRUST_PEM_FILE");
    println!(
        "AZ_IOT_DEVICE_X509_TRUST_PEM_FILE={}",
        global_provisioning_endpoint
    );

    println!("AZ_IOT_ID_SCOPE={}", id_scope);
    println!("AZ_IOT_REGISTRATION_ID_SAS={}", registration_id);
    println!("AZ_IOT_PROVISIONING_SAS_KEY={}", "***");
    println!(
        "AZ_IOT_PROVISIONING_SAS_KEY_DURATION_MINUTES={}",
        sas_key_duration
    );
    println!("AZ_IOT_DEVICE_X509_TRUST_PEM_FILE={:?}", trust_store);

    let options = client::ClientOptions::default();
    let client = client::Client::new(
        &global_provisioning_endpoint,
        &id_scope,
        &registration_id,
        Some(options),
    );
    //client
    let mut mqtt_client = connect_client_to_provisioning_service(&client, &sas_key, trust_store);

    subscribe_client_to_provisioning_service_topics(&mqtt_client);

    // Start listening before subscribing so that we don't miss messages
    let receiver_queue = mqtt_client.start_consuming();

    register_client_with_provisioning_service(&mqtt_client);

    let result = receive_registration_status(&receiver_queue, &mqtt_client);
    println!("Client received registration status.");

    disconnect_client_from_provisioning_service(&mqtt_client);
    if result.is_ok() {
        process::exit(0)
    } else {
        process::exit(1)
    }
}
