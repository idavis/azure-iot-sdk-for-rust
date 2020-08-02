use std::env;

fn main() {
    // set them in the build if they exist
    let set_env = "cargo:rustc-env";

    // Regenerate if any of these change
    println!("cargo:rerun-if-env-changed=AZ_IOT_GLOBAL_PROVISIONING_ENDPOINT");
    if let Ok(value) = env::var("AZ_IOT_GLOBAL_PROVISIONING_ENDPOINT") {
        println!("{}=AZ_IOT_GLOBAL_PROVISIONING_ENDPOINT={}", set_env, value);
    } else {
        // set the default if it wasn't specified
        println!(
            "{}=AZ_IOT_GLOBAL_PROVISIONING_ENDPOINT={}",
            set_env, "ssl://global.azure-devices-provisioning.net:8883"
        );
    }

    println!("cargo:rerun-if-env-changed=AZ_IOT_ID_SCOPE");
    if let Ok(value) = env::var("AZ_IOT_ID_SCOPE") {
        println!("{}=AZ_IOT_ID_SCOPE={}", set_env, value);
    }

    println!("cargo:rerun-if-env-changed=AZ_IOT_REGISTRATION_ID_SAS");
    if let Ok(value) = env::var("AZ_IOT_REGISTRATION_ID_SAS") {
        println!("{}=AZ_IOT_REGISTRATION_ID_SAS={}", set_env, value);
    }

    println!("cargo:rerun-if-env-changed=AZ_IOT_DEVICE_ID");
    if let Ok(value) = env::var("AZ_IOT_DEVICE_ID") {
        println!("{}=AZ_IOT_DEVICE_ID={}", set_env, value);
    }

    println!("cargo:rerun-if-env-changed=AZ_IOT_DEVICE_ID_SAS");
    if let Ok(value) = env::var("AZ_IOT_DEVICE_ID_SAS") {
        println!("{}=AZ_IOT_DEVICE_ID_SAS={}", set_env, value);
    }

    println!("cargo:rerun-if-env-changed=AZ_IOT_DEVICE_X509_TRUST_PEM_FILE");
    if let Ok(value) = env::var("AZ_IOT_DEVICE_X509_TRUST_PEM_FILE") {
        println!("{}=AZ_IOT_DEVICE_X509_TRUST_PEM_FILE={}", set_env, value);
    }

    println!("cargo:rerun-if-env-changed=AZ_IOT_DEVICE_X509_CERT_PEM_FILE");
    if let Ok(value) = env::var("AZ_IOT_DEVICE_X509_CERT_PEM_FILE") {
        println!("{}=AZ_IOT_DEVICE_X509_CERT_PEM_FILE={}", set_env, value);
    }

    println!("cargo:rerun-if-env-changed=AZ_IOT_PROVISIONING_SAS_KEY");
    if let Ok(value) = env::var("AZ_IOT_PROVISIONING_SAS_KEY") {
        println!("{}=AZ_IOT_PROVISIONING_SAS_KEY={}", set_env, value);
    }

    println!("cargo:rerun-if-env-changed=AZ_IOT_PROVISIONING_SAS_KEY_DURATION_MINUTES");
    if let Ok(value) = env::var("AZ_IOT_PROVISIONING_SAS_KEY_DURATION_MINUTES") {
        println!(
            "{}=AZ_IOT_PROVISIONING_SAS_KEY_DURATION_MINUTES={}",
            set_env, value
        );
    }

    println!("cargo:rerun-if-env-changed=AZ_IOT_HUB_HOSTNAME");
    if let Ok(value) = env::var("AZ_IOT_HUB_HOSTNAME") {
        println!("{}=AZ_IOT_HUB_HOSTNAME={}", set_env, value);
    }

    println!("cargo:rerun-if-env-changed=AZ_IOT_HUB_DEVICE_SAS_KEY");
    if let Ok(value) = env::var("AZ_IOT_HUB_DEVICE_SAS_KEY") {
        println!("{}=AZ_IOT_HUB_DEVICE_SAS_KEY={}", set_env, value);
    }

    println!("cargo:rerun-if-env-changed=AZ_IOT_HUB_DEVICE_SAS_KEY_DURATION_MINUTES");
    if let Ok(value) = env::var("AZ_IOT_HUB_DEVICE_SAS_KEY_DURATION_MINUTES") {
        println!(
            "{}=AZ_IOT_HUB_DEVICE_SAS_KEY_DURATION_MINUTES={}",
            set_env, value
        );
    }
}
