extern crate base64;

use self::base64::{encode};
use reqwest::header;
use sbanken::config::{get_config};

pub enum AuthenticationType {
    Basic,
    Bearer,
}

pub fn base64_encode_uname_pw(username: String, password: String) -> String {
    return encode(&format!("{}:{}", username, password));
}

pub fn generate_auth_header(credentials: String, auth_type: AuthenticationType) -> header::HeaderValue {
    let auth_header = match auth_type {
        AuthenticationType::Basic => format!("Basic {}", credentials),
        AuthenticationType::Bearer => format!("Bearer {}", credentials),
    };

    return generate_header_value(auth_header);
}

pub fn generate_header_value(value: String) -> header::HeaderValue {
    return match header::HeaderValue::from_str(&value) {
        Ok(val) => val,
        Err(error) => {
            panic!(format!("Failed to generate HTTP header: {}", error));
        }
    };
}

pub fn build_authorization_http_client() -> reqwest::Client {
    debug!("Building HTTP Client for authorization");
    
    let config = get_config();
    let auth_header = generate_auth_header(
        base64_encode_uname_pw(config.username, config.password),
        AuthenticationType::Basic);

    let mut headers = header::HeaderMap::new();
    headers.insert(header::AUTHORIZATION, auth_header);

    return match reqwest::Client::builder()
        .default_headers(headers)
        .build() {
            Ok(client) => client,
            Err(error) => {
                error!("Building HTTP Client for authorization failed: {}", error);
                panic!("building client failed");
            },
        };
}

pub fn build_api_client(credentials: String) -> reqwest::Client {
    debug!("Building HTTP Client for authorized API requests");
    let config = get_config();

    let auth_header = generate_auth_header(
        credentials,
        AuthenticationType::Bearer);

    let customer_id_header = generate_header_value(config.customer_id);

    let mut headers = header::HeaderMap::new();
    headers.insert(header::AUTHORIZATION, auth_header);
    headers.insert("customerId", customer_id_header);


    return match reqwest::Client::builder()
        .default_headers(headers)
        .build() {
            Ok(client) => client,
            Err(error) => {
                error!("Building HTTP Client failed: {}", error);
                panic!("building client failed");
            },
        };;
}
